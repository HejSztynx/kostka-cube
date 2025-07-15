use super::geometry::Point3D;

const CUBE_SIZE: f32 = 2.0;

#[derive(Clone, Copy)]
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
    pub color: Color,
}

impl Face {
    pub fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

pub struct Cubie {
    position: Point3D,
    rotation_y: f32,
    rotation_x: f32,
}

impl Cubie {
    pub fn new(position: (f32, f32, f32), rotation_y: f32, rotation_x: f32) -> Cubie {
        let (x, y, z) = position;
        Cubie {
            position: Point3D {x, y, z},
            rotation_y,
            rotation_x
        }
    }

    pub fn initial_corners(&self) -> [Point3D; 8] {
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

    pub fn transformed_corners(&self) -> Vec<Point3D> {
        self.initial_corners()
            .into_iter()
            .map(|p| p.rotate_y(self.rotation_y).rotate_x(self.rotation_x).translate(self.position))
            .collect()
    }

    pub fn faces(&self) -> Vec<Face> {
        let corners: Vec<Point3D> = self.transformed_corners();

        vec![
            Face{corners: [corners[0], corners[1], corners[2], corners[3]], color: Color::White},
            Face{corners: [corners[0], corners[3], corners[7], corners[4]], color: Color::Blue},
            Face{corners: [corners[1], corners[0], corners[4], corners[5]], color: Color::Red},
            Face{corners: [corners[2], corners[1], corners[5], corners[6]], color: Color::Green},
            Face{corners: [corners[3], corners[2], corners[6], corners[7]], color: Color::Orange},
            Face{corners: [corners[5], corners[4], corners[7], corners[6]], color: Color::Yellow},
        ]
    }
}