use core::f32;
use std::collections::HashMap;

use crate::{
    utils::{
        geometry::{snap_rotation, Point3D}
    },
    cube::{
        slice::CubeMove,
        core::{
            grid::{Grid, GridFace, GridSide},
        }
    },
    game::render::{AnyFace, Renderable}
};

const CUBE_SIZE: f32 = 2.0;
const TIEBRAKER_ROTATION: f32 = 0.015625;

#[derive(Clone)]
pub struct Face {
    pub corners: [Point3D; 4],
    pub markers: Vec<Point3D>,
    pub grid_face: GridFace,
}

impl Face {
    pub fn new(corners: [Point3D; 4], grid_face: GridFace) -> Face {
        let mut markers = Vec::with_capacity(16);
        let diff = corners[3].subtract(&corners[0]).scalar_multiply(1.0 / 3.0);
        for i in 0..4 {
            Self::create_markers(&mut markers, 
                corners[0].add(&diff.scalar_multiply(i as f32)), 
                corners[1].add(&diff.scalar_multiply(i as f32)));
        }

        Face { corners, markers, grid_face }
    }

    fn create_markers(markers: &mut Vec<Point3D>, v1: Point3D, v2: Point3D) {
        let diff = v2.subtract(&v1).scalar_multiply(1.0 / 3.0);
        for i in 0..4 {
            markers.push(v1.add(&diff.scalar_multiply(i as f32))); 
        }
    }

    pub fn center(&self) -> Point3D {
        let mut sum = Point3D { x: 0.0, y: 0.0, z: 0.0 };
        for p in &self.corners {
            sum.x += p.x;
            sum.y += p.y;
            sum.z += p.z;
        }
        Point3D {
            x: sum.x / 4.0,
            y: sum.y / 4.0,
            z: sum.z / 4.0,
        }
    }

    pub fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

pub struct Cube {
    position: Point3D,
    rotation_y: f32,
    rotation_x: f32,
    pub faces: Vec<Face>,
    side_map: HashMap<GridSide, GridSide>,
}

impl Cube {
    pub fn new(position: (f32, f32, f32), rotation_y: f32, rotation_x: f32) -> Cube {
        let (x, y, z) = position;
        let mut cube = Cube {
            position: Point3D {x, y, z},
            rotation_y,
            rotation_x,
            faces: vec![],
            side_map: HashMap::new(),
        };
        cube.update_side_map();
        cube
    }

    pub fn update_side_map(&mut self) {
        let mut side_map = HashMap::new();

        self.rotation_x -= TIEBRAKER_ROTATION;
        self.rotation_y += TIEBRAKER_ROTATION;
        self.apply_rotation();

        for &side in &[GridSide::Right, GridSide::Left, GridSide::Top, GridSide::Bottom, GridSide::Front, GridSide::Back] {
            let (best_face_idx, _) = self.faces
                .iter()
                .enumerate()
                .map(|(i, face)| {
                    let center = face.center();
                    let value = match side {
                        GridSide::Right => center.x,
                        GridSide::Left => -center.x,
                        GridSide::Top => center.y,
                        GridSide::Bottom => -center.y,
                        GridSide::Front => -center.z,
                        GridSide::Back => center.z,
                        _ => panic!()
                    };
                    (i, value)
                })
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let actual_side = GridSide::from_idx(best_face_idx);
            side_map.insert(side, actual_side);
        }

        self.rotation_x += TIEBRAKER_ROTATION;
        self.rotation_y -= TIEBRAKER_ROTATION;

        self.apply_rotation();
        self.side_map = side_map;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation_y = snap_rotation(self.rotation_y + angle);
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation_x = snap_rotation(self.rotation_x + angle);
    }

    fn try_get_grid_face_at_idx(&self, idx: usize) -> GridFace {
        if self.faces.len() == 0 {
            GridFace::empty()
        } else {
            self.faces[idx].grid_face.clone()
        }
    }

    fn apply_rotation(&mut self) {
        let corners: Vec<Point3D> = self.transformed_corners();

        let face_0 = self.try_get_grid_face_at_idx(0);
        let face_1 = self.try_get_grid_face_at_idx(1);
        let face_2 = self.try_get_grid_face_at_idx(2);
        let face_3 = self.try_get_grid_face_at_idx(3);
        let face_4 = self.try_get_grid_face_at_idx(4);
        let face_5 = self.try_get_grid_face_at_idx(5);

        self.faces = vec![
            Face::new([corners[2], corners[3], corners[0], corners[1]], face_0),
            Face::new([corners[2], corners[1], corners[5], corners[6]], face_1),
            Face::new([corners[1], corners[0], corners[4], corners[5]], face_2),
            Face::new([corners[0], corners[3], corners[7], corners[4]], face_3),
            Face::new([corners[3], corners[2], corners[6], corners[7]], face_4),
            Face::new([corners[5], corners[4], corners[7], corners[6]], face_5),
        ];
    }

    pub fn apply_grid(&mut self, grid: &Grid) {
        self.faces[0].grid_face = grid.faces[0].clone();
        self.faces[1].grid_face = grid.faces[1].clone();
        self.faces[2].grid_face = grid.faces[2].clone();
        self.faces[3].grid_face = grid.faces[3].clone();
        self.faces[4].grid_face = grid.faces[4].clone();
        self.faces[5].grid_face = grid.faces[5].clone();
    }

    fn initial_corners(&self) -> [Point3D; 8] {
        let h = CUBE_SIZE / 2.0;
        [
            Point3D { x: h, y: h, z: -h },
            Point3D { x: -h, y: h, z: -h },
            Point3D { x: -h, y: h, z: h },
            Point3D { x: h, y: h, z: h },
            
            Point3D { x: h, y: -h, z: -h },
            Point3D { x: -h, y: -h, z: -h },
            Point3D { x: -h, y: -h, z: h },
            Point3D { x: h, y: -h, z: h },
        ]
    }

    fn transformed_corners(&self) -> Vec<Point3D> {
        self.initial_corners()
            .into_iter()
            .map(|p| p.rotate_y(self.rotation_y).rotate_x(self.rotation_x).translate(self.position).snap())
            .collect()
    }

    pub fn translate_move(&self, cube_move: CubeMove) -> CubeMove {
        let side = cube_move.grid_side;
        let mut direction = cube_move.direction;
        let translated = self.side_map.get(&side.middle_layer_adjacent()).unwrap().clone();
        if side.is_middle() {
            let translated_middle = GridSide::middle_layer_from_axis(&translated.axis());
            if translated.idx() != translated_middle.middle_layer_adjacent().idx() {
                direction = direction.flip();
            }
            CubeMove::from_side(translated_middle, direction)
        } else {
            CubeMove::from_side(translated, direction)
        }
    }
}

impl Renderable for Cube {
    fn get_visible_faces(&self) -> Vec<AnyFace> {
        let mut faces_clone: Vec<AnyFace> = self.faces.clone()
            .into_iter()
            .map(|f| AnyFace::Face(f))
            .collect();

        faces_clone.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());
        faces_clone
    }

    fn dist(&self) -> f32 {
        let mut sum = 0.0;
        for face in &self.faces {
            sum += face.avg_z();
        }

        sum / 6.0
    }
}