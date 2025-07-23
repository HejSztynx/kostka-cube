use core::f32;
use std::collections::HashMap;

use crate::{
    cube_utils::{Axis, Color},
    grid::{Grid, GridFace, GridSide, NeighborSlice},
    screen::{AnyFace, Renderable},
    slice::{CubeSlice, CubeSliceOrder},
    geometry::Point3D,
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
    fn new(corners: [Point3D; 4], grid_face: GridFace) -> Face {
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
    faces: Vec<Face>,
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

        for &side in &[GridSide::RIGHT, GridSide::LEFT, GridSide::TOP, GridSide::BOTTOM, GridSide::FRONT, GridSide::BACK] {
            let (best_face_idx, _) = self.faces
                .iter()
                .enumerate()
                .map(|(i, face)| {
                    let center = face.center();
                    let value = match side {
                        GridSide::RIGHT => center.x,
                        GridSide::LEFT => -center.x,
                        GridSide::TOP => center.y,
                        GridSide::BOTTOM => -center.y,
                        GridSide::FRONT => -center.z,
                        GridSide::BACK => center.z,
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
        self.rotation_y += angle;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation_x += angle;
    }

    fn apply_rotation(&mut self) {
        let corners: Vec<Point3D> = self.transformed_corners();

        self.faces = vec![
            Face::new([corners[2], corners[3], corners[0], corners[1]], GridFace::empty()),
            Face::new([corners[2], corners[1], corners[5], corners[6]], GridFace::empty()),
            Face::new([corners[1], corners[0], corners[4], corners[5]], GridFace::empty()),
            Face::new([corners[0], corners[3], corners[7], corners[4]], GridFace::empty()),
            Face::new([corners[3], corners[2], corners[6], corners[7]], GridFace::empty()),
            Face::new([corners[5], corners[4], corners[7], corners[6]], GridFace::empty()),
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
            .map(|p| p.rotate_y(self.rotation_y).rotate_x(self.rotation_x).translate(self.position))
            .collect()
    }

    pub fn translate_side(&self, side: GridSide) -> GridSide {
        self.side_map.get(&side).unwrap().clone()
    }

    pub fn create_cube_slices(&self, grid: &Grid, axis: &Axis) -> [CubeSlice; 3] {
        let builder: CubeSliceBuilder = match axis {
            Axis::X => CubeSliceBuilder {
                cube: &self,
                split_faces: (
                    GridSide::TOP,
                    GridSide::BOTTOM
                ),
                idx_1: (13, 1),
                idx_2: (13, 1),
                idx_3: (2, 14),
                idx_4: (2, 14),
                face_1: GridSide::LEFT,
                face_2: GridSide::RIGHT,
            },
            Axis::Y => CubeSliceBuilder {
                cube: &self,
                split_faces: (
                    GridSide::BACK,
                    GridSide::FRONT
                ),
                idx_1: (4, 7),
                idx_2: (4, 7),
                idx_3: (11, 8),
                idx_4: (11, 8),
                face_1: GridSide::TOP,
                face_2: GridSide::BOTTOM,
            },
            Axis::Z => CubeSliceBuilder {
                cube: &self,
                split_faces: (
                    GridSide::TOP,
                    GridSide::BOTTOM
                ),
                idx_1: (11, 8),
                idx_2: (4, 7),
                idx_3: (4, 7),
                idx_4: (11, 8),
                face_1: GridSide::FRONT,
                face_2: GridSide::BACK,
            },
        };

        builder.build_cube_slices(grid)
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

struct CubeSliceBuilder<'a> {
    cube: &'a Cube,
    split_faces: (GridSide, GridSide),
    idx_1: (usize, usize),
    idx_2: (usize, usize),
    idx_3: (usize, usize),
    idx_4: (usize, usize),
    face_1: GridSide,
    face_2: GridSide,
}

impl <'a> CubeSliceBuilder<'a> {
    fn build_cube_slices(self, grid: &Grid) -> [CubeSlice; 3] {
        let sf_0_idx = self.split_faces.0.idx();
        let sf_2_idx = self.split_faces.1.idx();

        let f_1_idx = self.face_1.idx();
        let f_2_idx = self.face_2.idx();

        let axis = &self.face_1.axis();

        let mut last_corners = [
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.0).unwrap().clone(),
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.1).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.0).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.1).unwrap().clone(),
        ];

        if let Axis::Y = axis {
            last_corners.rotate_right(2);
        }

        let neighbors_1 = grid.get_neighbors(self.face_1);
        let neighbors_1_colors: Vec<[Color; 3]> = self.get_slices_colors(neighbors_1, grid);
        
        let middles = grid.get_middle_slices(axis);
        let middles_colors: Vec<[Color; 3]> = self.get_slices_colors(middles, grid);

        let neighbors_2 = grid.get_neighbors(self.face_2);
        let neighbors_2_colors: Vec<[Color; 3]> = self.get_slices_colors(neighbors_2, grid);

        [
            CubeSlice::new(
                self.cube.faces[f_1_idx].clone(),
                Face::new(
                    [
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.0).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.1).unwrap().clone(),
                    ], GridFace::empty()
                ),
                neighbors_1_colors,
                axis,
                CubeSliceOrder::FIRST
            ),
            CubeSlice::new(
                Face::new(
                    [
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.1).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.0).unwrap().clone(),
                    ], GridFace::empty()
                ),
                Face::new([
                        self.cube.faces[sf_0_idx].markers.get(self.idx_3.1).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_3.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_4.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_4.0).unwrap().clone(),
                    ], GridFace::empty()
                ),
                middles_colors,
                axis,
                CubeSliceOrder::MIDDLE
            ),
            CubeSlice::new(
                Face::new(last_corners, GridFace::empty()),
                self.cube.faces[f_2_idx].clone(),
                neighbors_2_colors,
                axis,
                CubeSliceOrder::LAST
            ),
        ]
    }

    fn get_slices_colors(&self, slices: [NeighborSlice; 4], grid: &Grid) -> Vec<[Color; 3]> {
        slices.iter()
            .map(|ns| ns.read_from(grid))
            .collect()
    }
}