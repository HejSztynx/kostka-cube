use std::{thread, time::Duration};
use std::io::{self, Write};

const CUBE_SIZE: f32 = 1.0;
// const SQRT_3: f32 = 1.73205;
const SQRT_2: f32 = 1.41421;

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 25;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

struct Cubie {
    position: Position,
    rotation: f32,
}

impl Cubie {
    fn new(position: (f32, f32, f32), rotation: f32) -> Cubie {
        let (x, y, z) = position;
        Cubie {
            position: Position {x, y, z},
            rotation
        }
    }

    fn corners(&self) -> Vec<Position> {
        let mut results = Vec::new();
        let half = CUBE_SIZE / 2.0;
        let (x, y, z) = (self.position.x, self.position.y, self.position.z);
        let y_p = y + half;
        let y_pp = y - half;

        let r = half * SQRT_2;
        let sin_r = self.rotation.sin() * r;
        let cos_r = self.rotation.cos() * r;

        let perpendicular = [(sin_r, cos_r), (cos_r, -sin_r)];
        for yy in [y_p, y_pp] {
            for perp in perpendicular {
                for d in [1.0, -1.0] {
                    results.push(Position{
                        x: x + d * perp.0,
                        y: yy,
                        z: z + d * perp.1
                    });
                }
            }
        }
        // println!("{}", self.rotation);

        results
    }
}

struct Screen {
    screen: [[bool; SCREEN_X]; SCREEN_Y],
    zp: f32,
    something: f32,
}

impl Screen {
    fn new(zp: f32, something: f32) -> Screen {
        Screen {
            screen: [[false; SCREEN_X]; SCREEN_Y],
            zp,
            something
        }
    }

    fn render_cubie(&mut self, cubie: &Cubie) {
        let corners = cubie.corners();
        for corner in corners {
            // println!("{:?}", corner);
            let position = corner;
            let multiplier = self.zp / position.z;
            let xp = position.x * multiplier;
            let yp = position.y * multiplier;
    
            let x_proj = (xp * self.something) as usize;
            let y_proj = (yp * self.something) as usize;

            self.screen[y_proj][x_proj] = true;
        }
    }

    fn print_screen(&self) {
        for y in (0..SCREEN_Y).rev() {
            for x in 0..SCREEN_X {
                let pixel = if self.screen[y][x] { '#' } else { ' ' };
                print!("{}", pixel);
            }
            println!();
        }
    }

    // fn clear_screen() {
        // print!("\x1B[2J\x1B[1;1H");
    // }

    fn move_cursor_up() {
        // print!("\x1B[{}A", SCREEN_Y);
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
