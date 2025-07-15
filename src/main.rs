use std::{thread, time::Duration};
use std::io::{self, Write};

const CUBE_SIZE: f32 = 2.0;
const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 5.0;
const PROJECTION_SCALE: f32 = 10.0;

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 45;

const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Triangle (
    Point2D,
    Point2D,
    Point2D,
);

impl Point3D {
    fn rotate_y(self, angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    fn rotate_x(self, angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    fn translate(self, offset: Point3D) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

#[derive(Clone, Copy)]
enum Color {
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
    fn to_ansi(&self) -> &str {
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

struct Face {
    corners: [Point3D; 4],
    color: Color,
}

impl Face {
    fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

struct Cubie {
    position: Point3D,
    rotation_y: f32,
    rotation_x: f32,
}

impl Cubie {
    fn new(position: (f32, f32, f32), rotation_y: f32, rotation_x: f32) -> Cubie {
        let (x, y, z) = position;
        Cubie {
            position: Point3D {x, y, z},
            rotation_y,
            rotation_x
        }
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

    fn faces(&self) -> Vec<Face> {
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

fn point_in_triangle(p: Point2D, a: Point2D, b: Point2D, c: Point2D) -> bool {
    let area = |p1: Point2D, p2: Point2D, p3: Point2D| -> isize {
        (p1.x * (p2.y - p3.y) +
         p2.x * (p3.y - p1.y) +
         p3.x * (p1.y - p2.y)).abs()
    };

    let total = area(a, b, c);
    let a1 = area(p, b, c);
    let a2 = area(a, p, c);
    let a3 = area(a, b, p);

    a1 + a2 + a3 <= total + 1
}


struct Screen {
    screen: [[Option<Color>; SCREEN_X]; SCREEN_Y],
    zp: f32,
    projection_scale: f32,
}

impl Screen {
    fn new(zp: f32, projection_scale: f32) -> Screen {
        Screen {
            screen: [[None; SCREEN_X]; SCREEN_Y],
            zp,
            projection_scale
        }
    }

    fn project_corner(&self, p: Point3D) -> Point2D {
        let multiplier = self.zp / p.z;
        let xp = p.x * multiplier;
        let yp = p.y * multiplier;

        let x_proj = (xp * self.projection_scale) as isize + 30;
        let y_proj = (yp * self.projection_scale) as isize + 22;

        Point2D {x: x_proj, y: y_proj}
    }

    fn rasterize_triangle(&mut self, Triangle(a, b, c): Triangle, color: Color) {
        let min_x = a.x.min(b.x.min(c.x)).max(0);
        let max_x = a.x.max(b.x.max(c.x)).min(SCREEN_X as isize - 1);
        let min_y = a.y.min(b.y.min(c.y)).max(0);
        let max_y = a.y.max(b.y.max(c.y)).min(SCREEN_Y as isize - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Point2D {x, y};
                if point_in_triangle(p, a, b, c) {
                    self.screen[y as usize][x as usize] = Some(color);
                }
            }
        }
    }

    fn render_face(&mut self, face: Face) {
        let projected: Vec<Point2D> = face.corners
            .iter()
            .map(|&p| self.project_corner(p))
            .collect();

        let tris = [
            Triangle(projected[0], projected[1], projected[2]),
            Triangle(projected[0], projected[2], projected[3]),
        ];

        for tri in tris {
            self.rasterize_triangle(tri, face.color);
        }

        // to remove after debug
        for proj in projected {
            self.screen[proj.y as usize][proj.x as usize] = Some(Color::Magenta);
        }
    }

    fn render_cubie(&mut self, cubie: &Cubie) {
        let mut faces = cubie.faces();
        faces.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());
        
        for face in faces.into_iter().rev() {
            self.render_face(face);
        }
    }

    fn print_screen(&self) {
        let char = "██";

        for y in (0..(SCREEN_Y)).rev() {
            for x in 0..(SCREEN_X) {
                match self.screen[y][x] {
                    Some(color) => print!("{}{char}{}", color.to_ansi(), ANSI_RESET),
                    _ => print!("  ")
                };
            }
            println!();
        }
        io::stdout().flush().unwrap();
    }

    fn clear_screen() {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}

fn cube() {
    let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    Screen::clear_screen();
    loop {
        let cubie = Cubie::new(position, angle_y, angle_x);
        
        let mut screen = Screen::new(ZP, PROJECTION_SCALE);
        
        screen.render_cubie(&cubie);
        screen.print_screen();
        thread::sleep(Duration::from_millis(100));
        Screen::clear_screen();

        angle_y += 0.1;
        angle_x += 0.1;
    }
}

fn main() {
    cube();
}