use std::io::{self, Write};

use crate::{
    utils::{
        cube_utils::Color,
        geometry::{Point2D, Point3D, Triangle}
    },
    cube::{
        slice::FaceSlice,
        cube::Face
    }
};

const SCREEN_X: usize = 70;
const SCREEN_Y: usize = 45;

const SCREEN_X_OFFSET: isize = 30;
const SCREEN_Y_OFFSET: isize = 22;

const PRINT_CHAR: &str = "██";
const ANSI_RESET: &str = "\x1b[0m";

pub enum AnyFace {
    Face(Face),
    FaceSlice(FaceSlice),
}

impl AnyFace {
    pub fn avg_z(&self) -> f32 {
        match self {
            AnyFace::Face(f) => f.avg_z(),
            AnyFace::FaceSlice(fs) => fs.avg_z(),
        }
    }
}

pub trait Renderable {
    fn get_visible_faces(&self) -> Vec<AnyFace>;

    fn dist(&self) -> f32;
}

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

    fn project_point(&self, p: Point3D) -> Point2D {
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

    fn render_face(&mut self, face: &Face) {
        let projected_markers: Vec<Point2D> = face.markers
            .iter()
            .map(|&p| self.project_point(p))
            .collect();

        for row in 0..3 {
            for col in 0..3 {
                let tris = [
                    Triangle(projected_markers.get(row * 4 + col).unwrap().clone(), 
                        projected_markers.get(row * 4 + col + 1).unwrap().clone(), 
                        projected_markers.get((row + 1) * 4 + col + 1).unwrap().clone()
                    ),
                    Triangle(projected_markers.get(row * 4 + col).unwrap().clone(), 
                        projected_markers.get((row + 1) * 4 + col + 1).unwrap().clone(),
                        projected_markers.get((row + 1) * 4 + col).unwrap().clone()
                    ),
                ];
                let color = face.grid_face.grid[row][col];
                for tri in tris {
                    self.rasterize_triangle(tri, color);
                }
            }
        }
    }

    fn render_face_slice(&mut self, face_slice: &FaceSlice) {
        let projected_markers: Vec<Point2D> = face_slice.markers
            .iter()
            .map(|&p| self.project_point(p))
            .collect();

        for row in 0..3 {
            let tris = [
                Triangle(projected_markers.get(row * 2).unwrap().clone(), 
                    projected_markers.get(row * 2 + 1).unwrap().clone(), 
                    projected_markers.get((row + 1) * 2 + 1).unwrap().clone()
                ),
                Triangle(projected_markers.get(row * 2).unwrap().clone(), 
                    projected_markers.get((row + 1) * 2 + 1).unwrap().clone(),
                    projected_markers.get((row + 1) * 2).unwrap().clone()
                ),
            ];
            let color = face_slice.colors[row];
            for tri in tris {
                self.rasterize_triangle(tri, color);
            }
        }
    }

    pub fn render(&mut self, mut renderables: Vec<&dyn Renderable>) {
        renderables.sort_by(|a, b| a.dist().partial_cmp(&b.dist()).unwrap());

        for renderable in renderables.into_iter().rev() {
            let faces = renderable.get_visible_faces();
            for face in faces.into_iter().take(3).rev() {
                match face {
                    AnyFace::Face(f) => self.render_face(&f),
                    AnyFace::FaceSlice(fs) => self.render_face_slice(&fs),
                };
            }
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

    pub fn clear_screen(&mut self) {
        for row in self.screen.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }

        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}