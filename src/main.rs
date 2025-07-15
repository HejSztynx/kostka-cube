use std::{thread, time::Duration};
use std::io::{self, Write};

const CUBE_SIZE: f32 = 2.0;
const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 5.0;
const SOMETHING: f32 = 10.0;


// const SQRT_3: f32 = 1.73205;
const SQRT_2: f32 = 1.41421;

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 45;

const ANSI_RESET: &str = "\x1b[0m";


#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
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

    fn translate(self, offset: Position) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}


struct Cubie {
    position: Position,
    rotation_y: f32,
    rotation_x: f32,
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
    corners: [Position; 4],
    color: Color,
}

impl Face {
    fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

impl Cubie {
    fn new(position: (f32, f32, f32), rotation_y: f32, rotation_x: f32) -> Cubie {
        let (x, y, z) = position;
        Cubie {
            position: Position {x, y, z},
            rotation_y,
            rotation_x
        }
    }

    fn faces(&self) -> Vec<Face> {
        let h = CUBE_SIZE / 2.0;
        // let r = half * SQRT_2;
        // let (x, y, z) = (self.position.x, self.position.y, -r);
        // let y_top = y + half;
        // let y_bot = y - half;

        let initial_corners = [
            Position { x: h, y: h, z: -h },
            Position { x: -h, y: h, z: -h },
            Position { x: -h, y: h, z: h },
            Position { x: h, y: h, z: h },
            
            Position { x: h, y: -h, z: -h },
            Position { x: -h, y: -h, z: -h },
            Position { x: -h, y: -h, z: h },
            Position { x: h, y: -h, z: h },
        ];

        let corners: Vec<Position> = initial_corners
            .into_iter()
            .map(|p| p.rotate_y(self.rotation_y).rotate_x(self.rotation_x).translate(self.position))
            .collect();

        // let corners = [
        //     Position{x: x * rot_y_cos + z * rot_y_sin, y: y_top, z: self.position.z + -x * rot_y_sin + z * rot_y_cos},
        //     Position{x: x * -rot_y_sin + z * rot_y_cos, y: y_top, z: self.position.z + -x * rot_y_cos + z * -rot_y_sin},
        //     Position{x: x * -rot_y_cos + z * -rot_y_sin, y: y_top, z: self.position.z + -x * -rot_y_sin + z * -rot_y_cos},
        //     Position{x: x * rot_y_sin + z * -rot_y_cos, y: y_top, z: self.position.z + -x * -rot_y_cos + z * rot_y_sin},
            
        //     Position{x: x * rot_y_cos + z * rot_y_sin, y: y_bot, z: self.position.z + -x * rot_y_sin + z * rot_y_cos},
        //     Position{x: x * -rot_y_sin + z * rot_y_cos, y: y_bot, z: self.position.z + -x * rot_y_cos + z * -rot_y_sin},
        //     Position{x: x * -rot_y_cos + z * -rot_y_sin, y: y_bot, z: self.position.z + -x * -rot_y_sin + z * -rot_y_cos},
        //     Position{x: x * rot_y_sin + z * -rot_y_cos, y: y_bot, z: self.position.z + -x * -rot_y_cos + z * rot_y_sin},
        // ];

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

#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: isize,
    y: isize,
}

struct Screen {
    screen: [[Option<Color>; SCREEN_X]; SCREEN_Y],
    zp: f32,
    something: f32,
}

impl Screen {
    fn new(zp: f32, something: f32) -> Screen {
        Screen {
            screen: [[None; SCREEN_X]; SCREEN_Y],
            zp,
            something
        }
    }

    fn render_cubie(&mut self, cubie: &Cubie) {
        let mut faces = cubie.faces();
        faces.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());

        // for face in faces.iter() {
            // println!("avg = {}", face.avg_z());
        // }
        // println!("--------------");
        
        for face in faces.into_iter().rev() {
            // self.print_screen();
            // std::thread::sleep(Duration::from_millis(1000));

            // println!("avg = {}", face.avg_z());
            let corners = face.corners; 

            // fill faces
            let mut projected: [Point2D; 4] = [Point2D {x: 0, y: 0}; 4];
            for (i, corner) in corners.iter().enumerate() {
                let multiplier = self.zp / corner.z;
                let xp = corner.x * multiplier;
                let yp = corner.y * multiplier;

                // println!("{} : {}", xp, yp);
                let x_proj = (xp * self.something) as isize + 30;
                let y_proj = (yp * self.something) as isize + 22;

                // println!("{} : {}", x_proj, y_proj);

                projected[i] = Point2D {x: x_proj, y: y_proj};
                // self.screen[y_proj as usize][x_proj as usize] = Some(Color::Green);
            }

            let tris = [
                (projected[0], projected[1], projected[2]),
                (projected[0], projected[2], projected[3]),
            ];

            for (a, b, c) in tris {
                let min_x = a.x.min(b.x.min(c.x)).max(0);
                let max_x = a.x.max(b.x.max(c.x)).min(SCREEN_X as isize - 1);
                let min_y = a.y.min(b.y.min(c.y)).max(0);
                let max_y = a.y.max(b.y.max(c.y)).min(SCREEN_Y as isize - 1);

                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let p = Point2D {x, y};
                        if point_in_triangle(p, a, b, c) {
                            self.screen[y as usize][x as usize] = Some(face.color);
                        }
                    }
                }
            }

            for proj in projected {
                let x_proj = proj.x;
                let y_proj = proj.y;
                self.screen[y_proj as usize][x_proj as usize] = Some(Color::Magenta);
            }

        }
    }

    fn print_screen(&self) {
        let char = "██";
        // let char = "##";
        let window = 0;
        for y in (window..(SCREEN_Y-window)).rev() {
            for x in window..(SCREEN_X-window) {
                match self.screen[y][x] {
                    Some(color) => print!("{}{char}{}", color.to_ansi(), ANSI_RESET),
                    // _ => print!("{}{char}{}", Color::White.to_ansi(), ANSI_RESET),
                    _ => print!("  ")
                };
            }
            println!();
        }
    }

    fn move_cursor_up() {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}

fn cube() {
    let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    // let mut somet = 3.0;
    loop {
        // Screen::clear_screen();
        let cubie = Cubie::new(position, angle_y, angle_x);
        
        let mut screen = Screen::new(ZP, SOMETHING);
        
        screen.render_cubie(&cubie);
        thread::sleep(Duration::from_millis(100));
        Screen::move_cursor_up();
        screen.print_screen();

        angle_y += 0.1;
        angle_x += 0.1;
        // somet += 0.1;
    }
}

fn main() {
    // test();
    
    cube();
}

fn test() {
    println!("siema siema\nsienamsi\nasdas\n\nasdasd\nasdas");
    
    thread::sleep(Duration::from_millis(500));
    Screen::move_cursor_up();
    thread::sleep(Duration::from_millis(500));
    println!("halo\nhalo\nhalo\n\neeee");
}
