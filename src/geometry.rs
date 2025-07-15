#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle (
    pub Point2D,
    pub Point2D,
    pub Point2D,
);

impl Triangle {
    pub fn point_in_triangle(&self, p: Point2D) -> bool {
        let Self(a, b, c) = self;
        let area = |p1: &Point2D, p2: &Point2D, p3: &Point2D| -> isize {
            (p1.x * (p2.y - p3.y) +
            p2.x * (p3.y - p1.y) +
            p3.x * (p1.y - p2.y)).abs()
        };

        let total = area(a, b, c);
        let a1 = area(&p, b, c);
        let a2 = area(a, &p, c);
        let a3 = area(a, b, &p);

        a1 + a2 + a3 <= total + 1
    }
}

impl Point3D {
    pub fn rotate_y(self, angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    pub fn translate(self, offset: Point3D) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

