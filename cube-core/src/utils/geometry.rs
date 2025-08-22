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

impl Point3D {
    pub fn add(&self, other: &Point3D) -> Point3D {
        Point3D { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z, 
        }
    }

    pub fn subtract(&self, other: &Point3D) -> Point3D {
        Point3D { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z, 
        }
    }

    pub fn scalar_multiply(&self, scalar: f32) -> Point3D {
        Point3D { 
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z, 
        }
    }

    pub fn rotate_around_axis(&self, axis: Point3D, origin: Point3D, angle_rad: f32) -> Point3D {
        let axis = axis.normalize();
        let v = self.subtract(&origin);
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();

        let dot = v.dot(&axis);
        let cross = v.cross(&axis);

        let rotated = v.scalar_multiply(cos)
            .add(&cross.scalar_multiply(sin))
            .add(&axis.scalar_multiply(dot * (1.0 - cos)));

        rotated.add(&origin)
    }

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

    pub fn rotate_z(self, angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }

    pub fn translate(self, offset: Point3D) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }

    pub fn dot(&self, other: &Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point3D) -> Point3D {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Point3D {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Point3D {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
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