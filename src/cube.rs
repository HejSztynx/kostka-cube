use std::pin::Pin;

use crate::grid::{Grid, GridFace};

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
            Color::Gray  => "\x1b[90m",
            Color::Orange => "\x1b[38;5;208m",
            Color::Magenta => "\x1b[95m",
            Color::Green => "\x1b[92m"
        }
    }
}

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

    pub fn get_visible_faces(&mut self) -> &Vec<Face> {
        self.faces.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());
        &self.faces
    }
}