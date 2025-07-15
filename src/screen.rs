use std::io::{self, Write};

use super::geometry::{Point2D, Point3D, Triangle};
use super::cube::{Color, Cubie, Face};

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 45;

const SCREEN_X_OFFSET: isize = 30;
const SCREEN_Y_OFFSET: isize = 22;

const PRINT_CHAR: &str = "██";
const ANSI_RESET: &str = "\x1b[0m";

pub struct Screen {
    screen: [[Option<Color>; SCREEN_X]; SCREEN_Y],
    zp: f32,
    projection_scale: f32,
}

impl Screen {
    pub fn new(zp: f32, projection_scale: f32) -> Screen {
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

        let x_proj = (xp * self.projection_scale) as isize + SCREEN_X_OFFSET;
        let y_proj = (yp * self.projection_scale) as isize + SCREEN_Y_OFFSET;

        Point2D {x: x_proj, y: y_proj}
    }

    fn rasterize_triangle(&mut self, triangle: Triangle, color: Color) {
        let Triangle(a, b, c) = triangle;
        let min_x = a.x.min(b.x.min(c.x)).max(0);
        let max_x = a.x.max(b.x.max(c.x)).min(SCREEN_X as isize - 1);
        let min_y = a.y.min(b.y.min(c.y)).max(0);
        let max_y = a.y.max(b.y.max(c.y)).min(SCREEN_Y as isize - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Point2D {x, y};
                if triangle.point_in_triangle(p) {
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

    pub fn render_cubie(&mut self, cubie: &Cubie) {
        let mut faces = cubie.faces();
        faces.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());
        
        for face in faces.into_iter().rev() {
            self.render_face(face);
        }
    }

    pub fn print_screen(&self) {
        for y in (0..(SCREEN_Y)).rev() {
            for x in 0..(SCREEN_X) {
                match self.screen[y][x] {
                    Some(color) => print!("{}{}{}", color.to_ansi(), PRINT_CHAR, ANSI_RESET),
                    _ => print!("  ")
                };
            }
            println!();
        }
        io::stdout().flush().unwrap();
    }

    pub fn clear_screen() {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}