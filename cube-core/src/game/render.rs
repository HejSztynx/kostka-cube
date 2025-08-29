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
    size_x: usize,
    size_y: usize,
    screen: Vec<Vec<Option<Color>>>,
    zp: f32,
    projection_scale: f32,
}

impl Screen {
    pub fn new(size_x: usize, size_y: usize, zp: f32, projection_scale: f32) -> Screen {
        Screen {
            size_x,
            size_y,
            screen: vec![vec![None; size_x]; size_y],
            zp,
            projection_scale,
        }
    }

    pub fn color_at(&self, x: i16, y: i16) -> Option<Color> {
        self.screen[self.size_y - y as usize - 1][x as usize]
    }

    fn project_point(&self, p: Point3D) -> Point2D {
        let screen_x_offset = (self.size_x / 2) as isize;
        let screen_y_offset = (self.size_y / 2) as isize;

        let multiplier = self.zp / p.z;
        let xp = p.x * multiplier;
        let yp = p.y * multiplier;

        let x_proj = (xp * self.projection_scale) as isize + screen_x_offset;
        let y_proj = (yp * self.projection_scale) as isize + screen_y_offset;

        Point2D {x: x_proj, y: y_proj}
    }

    // uses the "scanline" method
    fn rasterize_triangle(&mut self, tri: Triangle, color: Color) {
        let mut pts = [tri.0, tri.1, tri.2];
        // sort vertices by y (top to bottom)
        pts.sort_by_key(|p| p.y);

        let (v1, v2, v3) = (pts[0], pts[1], pts[2]);

        // avoid dividing by zero
        let inv_slope_1 = if v2.y != v1.y {
            (v2.x - v1.x) as f32 / (v2.y - v1.y) as f32
        } else { 0.0 };

        let inv_slope_2 = if v3.y != v1.y {
            (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32
        } else { 0.0 };

        let inv_slope_3 = if v3.y != v2.y {
            (v3.x - v2.x) as f32 / (v3.y - v2.y) as f32
        } else { 0.0 };

        // top half (v1 → v2 and v1 → v3)
        let mut curx1 = v1.x as f32;
        let mut curx2 = v1.x as f32;
        for y in v1.y..=v2.y {
            if y >= 0 && y < self.size_y as isize {
                let x_start = curx1.min(curx2).max(0.0) as isize;
                let x_end   = curx1.max(curx2).min(self.size_x as f32 - 1.0) as isize;
                for x in x_start..=x_end {
                    self.screen[y as usize][x as usize] = Some(color);
                }
            }
            curx1 += inv_slope_1;
            curx2 += inv_slope_2;
        }

        // bottom half (v2 → v3 and v1 → v3)
        let mut curx1 = v2.x as f32;
        let mut curx2 = v1.x as f32 + inv_slope_2 * (v2.y - v1.y) as f32;
        for y in v2.y..=v3.y {
            if y >= 0 && y < self.size_y as isize {
                let x_start = curx1.min(curx2).max(0.0) as isize;
                let x_end   = curx1.max(curx2).min(self.size_x as f32 - 1.0) as isize;
                for x in x_start..=x_end {
                    self.screen[y as usize][x as usize] = Some(color);
                }
            }
            curx1 += inv_slope_3;
            curx2 += inv_slope_2;
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
        for y in (0..(self.size_y)).rev() {
            for x in 0..(self.size_x) {
                match self.screen[y][x] {
                    Some(color) => print!("{}{}{}", color.to_ansi(), PRINT_CHAR, ANSI_RESET),
                    _ => print!("  ")
                };
            }
            println!();
        }
        io::stdout().flush().unwrap();
    }

    pub fn reset_terminal() {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }

    pub fn clear_screen(&mut self) {
        for row in self.screen.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
    }
}