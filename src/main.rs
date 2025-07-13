use std::{thread, time::Duration};
use std::io::{self, Write};

const CUBE_SIZE: f32 = 1.0;
// const SQRT_3: f32 = 1.73205;
const SQRT_2: f32 = 1.41421;

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 25;

const ANSI_RESET: &str = "\x1b[0m";


#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

struct Cubie {
    position: Position,
    rotation: f32,
}

#[derive(Clone, Copy)]
enum Color {
    White,
    Blue,
    Red,
    Gray,
}

impl Color {
    fn to_ansi(&self) -> &str {
        match self {
            Color::White => "\x1b[97m",
            Color::Blue  => "\x1b[94m",
            Color::Red   => "\x1b[91m",
            Color::Gray  => "\x1b[90m",
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
    fn new(position: (f32, f32, f32), rotation: f32) -> Cubie {
        let (x, y, z) = position;
        Cubie {
            position: Position {x, y, z},
            rotation
        }
    }

    fn faces(&self) -> Vec<Face> {
        let half = CUBE_SIZE / 2.0;
        let (x, y, z) = (self.position.x, self.position.y, self.position.z);
        let y_top = y + half;
        let y_bot = y - half;

        let r = half * SQRT_2;
        let sin_r = self.rotation.sin() * r;
        let cos_r = self.rotation.cos() * r;

        let corners = [
            Position{x: x + sin_r, y: y_top, z: z + cos_r},
            Position{x: x + cos_r, y: y_top, z: z - sin_r},
            Position{x: x - sin_r, y: y_top, z: z - cos_r},
            Position{x: x - cos_r, y: y_top, z: z + sin_r},

            Position{x: x + sin_r, y: y_bot, z: z + cos_r},
            Position{x: x + cos_r, y: y_bot, z: z - sin_r},
            Position{x: x - sin_r, y: y_bot, z: z - cos_r},
            Position{x: x - cos_r, y: y_bot, z: z + sin_r},
        ];

        // let perpendicular = [(sin_r, cos_r), (cos_r, -sin_r)];
        // let mut corners = vec![];
        // for yy in [y_top, y_bot] {
        //     for perp in perpendicular {
        //         for d in [1.0, -1.0] {
        //             corners.push(Position{
        //                 x: x + d * perp.0,
        //                 y: yy,
        //                 z: z + d * perp.1
        //             });
        //         }
        //     }
        // }
        // println!("{}", self.rotation);

        vec![
            Face{corners: [corners[0], corners[1], corners[2], corners[3]], color: Color::White},
            Face{corners: [corners[0], corners[3], corners[4], corners[7]], color: Color::Blue},
            Face{corners: [corners[0], corners[1], corners[4], corners[5]], color: Color::Red},
            Face{corners: [corners[1], corners[2], corners[5], corners[6]], color: Color::Gray},
            Face{corners: [corners[2], corners[3], corners[6], corners[7]], color: Color::Gray},
            Face{corners: [corners[4], corners[5], corners[6], corners[7]], color: Color::Gray},
        ]
    }
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
        
        for face in faces {
            let corners = face.corners;
            for corner in corners {
                // println!("{:?}", corner);
                let position = corner;
                let multiplier = self.zp / position.z;
                let xp = position.x * multiplier;
                let yp = position.y * multiplier;
                
                let x_proj = (xp * self.something) as usize;
                let y_proj = (yp * self.something) as usize;
                
                self.screen[y_proj][x_proj] = Some(face.color);
            }
        }
    }

    fn print_screen(&self) {
        for y in (0..SCREEN_Y).rev() {
            for x in 0..SCREEN_X {
                // let pixel = if self.screen[y][x] { '#' } else { ' ' };
                match self.screen[y][x] {
                    Some(color) => print!("{}#{}", color.to_ansi(), ANSI_RESET),
                    _ => print!(" ")
                };
                // print!("{}#{}", color.to_ansi(), ANSI_RESET);
                // print!("{}", pixel);
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
    let position: (f32, f32, f32) = (5.0, 2.0, 5.0);
    let mut angle = 0.0;

    // let mut somet = 3.0;
    loop {
        // Screen::clear_screen();
        let cubie = Cubie::new(position, angle);
        
        let mut screen = Screen::new(5.0, 8.0);
        
        screen.render_cubie(&cubie);
        screen.print_screen();

        angle += 0.1;
        // somet += 0.1;
        thread::sleep(Duration::from_millis(100));
        Screen::move_cursor_up();
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
