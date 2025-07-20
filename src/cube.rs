use crate::{grid::{Grid, GridFace, GridSide}, screen::{AnyFace, Renderable}, slice::CubeSlice};
use super::geometry::Point3D;

const CUBE_SIZE: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Yellow,
    Blue,
    Red,
    Green,
    Orange,
    Magenta,
    Gray,
}

impl Color {
    pub fn to_ansi(&self) -> &str {
        match self {
            Color::White => "\x1b[97m",
            Color::Yellow => "\x1b[93m",
            Color::Blue  => "\x1b[94m",
            Color::Red   => "\x1b[91m",
            Color::Green => "\x1b[92m",
            Color::Orange => "\x1b[38;5;208m",
            Color::Magenta => "\x1b[95m",
            Color::Gray  => "\x1b[90m",
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

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

    pub fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

pub struct Cube {
    position: Point3D,
    rotation_y: f32,
    rotation_x: f32,
    faces: Vec<Face>,
}

impl Cube {
    pub fn new(position: (f32, f32, f32), rotation_y: f32, rotation_x: f32) -> Cube {
        let (x, y, z) = position;
        Cube {
            position: Point3D {x, y, z},
            rotation_y,
            rotation_x,
            faces: vec![],
        }
    }

    pub fn apply_grid(&mut self, grid: &Grid) {
        let corners: Vec<Point3D> = self.transformed_corners();

        self.faces = vec![
            Face::new([corners[2], corners[3], corners[0], corners[1]], grid.faces[0].clone()),
            Face::new([corners[2], corners[1], corners[5], corners[6]], grid.faces[1].clone()),
            Face::new([corners[1], corners[0], corners[4], corners[5]], grid.faces[2].clone()),
            Face::new([corners[0], corners[3], corners[7], corners[4]], grid.faces[3].clone()),
            Face::new([corners[3], corners[2], corners[6], corners[7]], grid.faces[4].clone()),
            Face::new([corners[5], corners[4], corners[7], corners[6]], grid.faces[5].clone()),
        ];
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

    pub fn create_cube_slices(&self, grid: &Grid, axis: Axis) -> [CubeSlice; 3] {
        let builder: CubeSliceBuilder = match axis {
            Axis::X => CubeSliceBuilder {
                cube: &self,
                // split_faces: [
                //     &self.faces[0], // top
                //     &self.faces[4], // back
                //     &self.faces[5], // bottom
                //     &self.faces[2], // front
                // ],
                split_faces: [
                    GridSide::TOP, // top
                    GridSide::FRONT, // front
                    GridSide::BOTTOM, // bottom
                    GridSide::BACK, // back
                ],
                idx_1: (13, 1),
                idx_2: (13, 1),
                idx_3: (2, 14),
                idx_4: (2, 14),
                flip: false,
                // face_1: &self.faces[1], // left
                face_1: GridSide::LEFT, // left
                // face_2: &self.faces[3], // right
                face_2: GridSide::RIGHT, // right
            },
            Axis::Y => CubeSliceBuilder {
                cube: &self,
                // split_faces: [
                //     &self.faces[4], // back
                //     &self.faces[1], // left
                //     &self.faces[2], // front
                //     &self.faces[3], // right
                // ],
                split_faces: [
                    GridSide::BACK, // back
                    GridSide::RIGHT, // right
                    GridSide::FRONT, // front
                    GridSide::LEFT, // left
                ],
                idx_1: (4, 7),
                idx_2: (4, 7),
                idx_3: (11, 8),
                idx_4: (11, 8),
                flip: true,
                // face_1: &self.faces[0], // top
                face_1: GridSide::TOP,
                // face_2: &self.faces[5], // bottom
                face_2: GridSide::BOTTOM,
            },
            Axis::Z => CubeSliceBuilder {
                cube: &self,
                // split_faces: [
                //     &self.faces[0], // top
                //     &self.faces[1], // left
                //     &self.faces[5], // bottom
                //     &self.faces[3], // right
                // ],
                split_faces: [
                    GridSide::TOP, // top
                    GridSide::RIGHT, // right
                    GridSide::BOTTOM, // bottom
                    GridSide::LEFT, // left
                ],
                idx_1: (11, 8),
                idx_2: (4, 7),
                idx_3: (4, 7),
                idx_4: (11, 8),
                flip: false,
                // face_1: &self.faces[2], // front
                face_1: GridSide::FRONT,
                // face_2: &self.faces[4], // back
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
}

struct CubeSliceBuilder<'a> {
    cube: &'a Cube,
    split_faces: [GridSide; 4],
    idx_1: (usize, usize),
    idx_2: (usize, usize),
    idx_3: (usize, usize),
    idx_4: (usize, usize),
    flip: bool,
    face_1: GridSide,
    face_2: GridSide,
}

impl <'a> CubeSliceBuilder<'a> {
    fn build_cube_slices(self, grid: &Grid) -> [CubeSlice; 3] {
        let sf_0_idx = self.split_faces[0].idx();
        let sf_1_idx = self.split_faces[1].idx();
        let sf_2_idx = self.split_faces[2].idx();
        let sf_3_idx = self.split_faces[3].idx();

        let f_1_idx = self.face_1.idx();
        let f_2_idx = self.face_2.idx();

        let axis = &self.face_1.axis();

        let mut last_corners = [
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.0).unwrap().clone(),
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.1).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.0).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.1).unwrap().clone(),
        ];

        if self.flip {
            last_corners.rotate_right(2);
        }

        let neighbors_1 = grid.get_neighbors(self.face_1);
        // neighbors_1.rotate_right(1);
        let neighbors_1_colors: Vec<[Color; 3]> = neighbors_1.iter()
            .map(|ns| {
                let mut res = ns.read_from_render_ready(grid);
                // res.reverse();
                res
            })
            // .rev()
            .collect();

        for colors in neighbors_1_colors.iter() {
            println!("1: {:?}", colors);
        }
        
        let neighbors_2 = grid.get_neighbors(self.face_2);
        let mut neighbors_2_colors: Vec<[Color; 3]> = neighbors_2.iter()
            .map(|ns| ns.read_from_render_ready(grid))
            // .rev()
            .collect();
        // neighbors_2_colors.swap(1, 3);
        
        for colors in neighbors_2_colors.iter() {
            println!("2: {:?}", colors);
        }

        let magenta_colors = vec![
            [Color::Magenta, Color::Green, Color::Red],
            [Color::Magenta, Color::Green, Color::Red],
            [Color::Magenta, Color::Green, Color::Red],
            // [Color::Magenta, Color::Green, Color::Red],
            [Color::Blue, Color::Blue, Color::Blue],
        ];

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
                // magenta_colors.clone()
                neighbors_1_colors,
                axis,
                false
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
                magenta_colors.clone(),
                axis,
                false
            ),
            CubeSlice::new(
                Face::new(last_corners, GridFace::empty()),
                self.cube.faces[f_2_idx].clone(),
                // magenta_colors
                neighbors_2_colors,
                axis,
                self.flip
            ),
        ]
    }
}