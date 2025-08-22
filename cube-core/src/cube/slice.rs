use crate::{
    utils::{
        cube_utils::{Axis, Color},
        geometry::Point3D
    },
    cube::{
        cube::Face,
        core::{
            grid::{GridSide, MoveDirection}
        }
    },
    game::render::{AnyFace, Renderable}
};

const TIEBREAKER_WEIGHT: f32 = 0.001;

#[derive(Clone)]
pub struct FaceSlice {
    corners: [Point3D; 4],
    pub markers: Vec<Point3D>,
    pub colors: [Color; 3],
}

impl FaceSlice {
    fn new(corners: [Point3D; 4], colors: [Color; 3]) -> FaceSlice {
        let mut markers = Vec::with_capacity(8);
        let diff = corners[3].subtract(&corners[0]).scalar_multiply(1.0 / 3.0);
        for i in 0..4 {
            markers.push(corners[0].add(&diff.scalar_multiply(i as f32))); 
            markers.push(corners[1].add(&diff.scalar_multiply(i as f32)));
        };

        FaceSlice { corners, markers, colors }
    }

    fn avg_x(&self) -> f32 {
        (self.corners[0].x.abs() + self.corners[1].x.abs() + self.corners[2].x.abs() + self.corners[3].x.abs()) / 4.0
    }

    fn avg_y(&self) -> f32 {
        (self.corners[0].y.abs() + self.corners[1].y.abs() + self.corners[2].y.abs() + self.corners[3].y.abs()) / 4.0
    }

    pub fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

#[derive(Debug, Clone)]
pub struct CubeMove {
    pub axis: Axis,
    pub grid_side: GridSide,
    pub order: CubeSliceOrder,
    pub direction: MoveDirection,
}

impl CubeMove {
    pub fn from_side(grid_side: GridSide, direction: MoveDirection) -> CubeMove {
        CubeMove { axis: grid_side.axis(), grid_side, order: grid_side.order(), direction }
    }

    pub fn from_str(mv: &str) -> Result<(GridSide, MoveDirection), String> {
        let (side_char, suffix) = mv.split_at(1);

        let grid_side= match side_char {
            "R" => GridSide::Right,
            "L" => GridSide::Left,
            "U" => GridSide::Top,
            "D" => GridSide::Bottom,
            "F" => GridSide::Front,
            "B" => GridSide::Back,
            "M" => GridSide::MiddleX,
            "E" => GridSide::MiddleY,
            "S" => GridSide::MiddleZ,
            _ => return Err(format!("Incorrect move '{}'", mv)),
        };
        let direction = match suffix {
            "" => Ok(MoveDirection::Clockwise),
            "'" => Ok(MoveDirection::CounterClockwise),
            "2" => Ok(MoveDirection::Double),
            _ => Err(format!("Incorrect move '{}'", mv)),
        }?;

        Ok((grid_side, direction))
    }
}

#[derive(Debug, Clone)]
pub enum CubeSliceOrder {
    FIRST,
    MIDDLE,
    LAST,
}

impl CubeSliceOrder {
    pub fn idx(&self) -> usize {
        match self {
            Self::FIRST => 0,
            Self::MIDDLE => 1,
            Self::LAST => 2,
        }
    }
}

#[derive(Clone)]
pub struct CubeSlice {
    pub global_cube_position: Point3D,
    pub face_1: Face,
    pub face_2: Face,
    pub face_slices: [FaceSlice; 4],
}

impl CubeSlice {
    pub fn new(
        global_cube_position: Point3D,
        face_1: Face, 
        face_2: Face, 
        mut colors: Vec<[Color; 3]>, 
        axis: &Axis, 
        order: CubeSliceOrder
    ) -> CubeSlice {

        Self::flip_colors(&mut colors, axis, order);

        let face_slices = [
            FaceSlice::new(
                [
                    face_1.corners[0],
                    face_2.corners[1],
                    face_2.corners[0],
                    face_1.corners[1],
                ],
                *colors.get(0).unwrap()
            ),
            FaceSlice::new(
                [
                    face_1.corners[1],
                    face_2.corners[0],
                    face_2.corners[3],
                    face_1.corners[2],
                ],
                *colors.get(1).unwrap()
            ),
            FaceSlice::new(
                [
                    face_1.corners[2],
                    face_2.corners[3],
                    face_2.corners[2],
                    face_1.corners[3],
                ],
                *colors.get(2).unwrap()
            ),
            FaceSlice::new(
                [
                    face_1.corners[3],
                    face_2.corners[2],
                    face_2.corners[1],
                    face_1.corners[0],
                ],
                *colors.get(3).unwrap()
            ),
        ];

        CubeSlice { global_cube_position, face_1, face_2, face_slices }
    }

    fn flip_colors(
        colors: &mut Vec<[Color; 3]>, 
        axis: &Axis, 
        order: CubeSliceOrder
    ) {
        match axis {
            Axis::X => {
                if let CubeSliceOrder::LAST = &order { } else {
                    for i in 0..4 {
                        if let Some(color) = colors.get_mut(i) {
                            color.reverse();
                        }
                    }
                }
            },
            Axis::Y => {
                if let CubeSliceOrder::LAST = &order {
                    colors.rotate_right(2);
                } else {
                    for i in 0..4 {
                        if let Some(color) = colors.get_mut(i) {
                            color.reverse();
                        }
                    }
                }
            },
            Axis::Z => {
                if let CubeSliceOrder::LAST = &order { } else {
                    for i in 0..4 {
                        if let Some(color) = colors.get_mut(i) {
                            color.reverse();
                        }
                    }
                }
            }
        }
    }

    pub fn rotate_around_own_axis(&mut self, angle_rad: f32) {
        let center1 = self.face_1.center();
        let center2 = self.face_2.center();
        let axis = center2.subtract(&center1);

        for p in &mut self.face_1.corners {
            *p = p.rotate_around_axis(axis, center1, angle_rad);
        }
        for p in &mut self.face_1.markers {
            *p = p.rotate_around_axis(axis, center1, angle_rad);
        }

        for p in &mut self.face_2.corners {
            *p = p.rotate_around_axis(axis, center1, angle_rad);
        }
        for p in &mut self.face_2.markers {
            *p = p.rotate_around_axis(axis, center1, angle_rad);
        }

        for slice in &mut self.face_slices {
            for p in &mut slice.corners {
                *p = p.rotate_around_axis(axis, center1, angle_rad);
            }
            for p in &mut slice.markers {
                *p = p.rotate_around_axis(axis, center1, angle_rad);
            }
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let flipped_offset = self.global_cube_position.scalar_multiply(-1.0);
        for p in &mut self.face_1.corners {
            *p = p.translate(flipped_offset);
            *p = p.rotate_x(angle);
            *p = p.translate(self.global_cube_position);
        }
        for p in &mut self.face_1.markers {
            *p = p.translate(flipped_offset);
            *p = p.rotate_x(angle);
            *p = p.translate(self.global_cube_position);
        }
        
        for p in &mut self.face_2.corners {
            *p = p.translate(flipped_offset);
            *p = p.rotate_x(angle);
            *p = p.translate(self.global_cube_position);
        }
        for p in &mut self.face_2.markers {
            *p = p.translate(flipped_offset);
            *p = p.rotate_x(angle);
            *p = p.translate(self.global_cube_position);
        }
        
        for slice in &mut self.face_slices {
            for p in &mut slice.corners {
                *p = p.translate(flipped_offset);
                *p = p.rotate_x(angle);
                *p = p.translate(self.global_cube_position);
            }
            for p in &mut slice.markers {
                *p = p.translate(flipped_offset);
                *p = p.rotate_x(angle);
                *p = p.translate(self.global_cube_position);
            }
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let flipped_offset = self.global_cube_position.scalar_multiply(-1.0);
        for p in &mut self.face_1.corners {
            *p = p.translate(flipped_offset);
            *p = p.rotate_y(angle);
            *p = p.translate(self.global_cube_position);
        }
        for p in &mut self.face_1.markers {
            *p = p.translate(flipped_offset);
            *p = p.rotate_y(angle);
            *p = p.translate(self.global_cube_position);
        }
        
        for p in &mut self.face_2.corners {
            *p = p.translate(flipped_offset);
            *p = p.rotate_y(angle);
            *p = p.translate(self.global_cube_position);
        }
        for p in &mut self.face_2.markers {
            *p = p.translate(flipped_offset);
            *p = p.rotate_y(angle);
            *p = p.translate(self.global_cube_position);
        }
        
        for slice in &mut self.face_slices {
            for p in &mut slice.corners {
                *p = p.translate(flipped_offset);
                *p = p.rotate_y(angle);
                *p = p.translate(self.global_cube_position);
            }
            for p in &mut slice.markers {
                *p = p.translate(flipped_offset);
                *p = p.rotate_y(angle);
                *p = p.translate(self.global_cube_position);
            }
        }
    }

    // pub fn rotate_y(&mut self, angle: f32) {
    //     let axis = Point3D{
    //         x: 0.0, 
    //         y: 1.0, 
    //         z: 0.0
    //     };
    //     let center = self.global_cube_position; // bo to środek globalny kostki

    //     for p in &mut self.face_1.corners {
    //         *p = p.rotate_around_axis(axis, center, angle);
    //     }
    //     for p in &mut self.face_1.markers {
    //         *p = p.rotate_around_axis(axis, center, angle);
    //     }
    //     for p in &mut self.face_2.corners {
    //         *p = p.rotate_around_axis(axis, center, angle);
    //     }
    //     for p in &mut self.face_2.markers {
    //         *p = p.rotate_around_axis(axis, center, angle);
    //     }
    //     for slice in &mut self.face_slices {
    //         for p in &mut slice.corners {
    //             *p = p.rotate_around_axis(axis, center, angle);
    //         }
    //         for p in &mut slice.markers {
    //             *p = p.rotate_around_axis(axis, center, angle);
    //         }
    //     }
    // }


    // pub fn rotate_y(&mut self, angle: f32) {
    //     for p in &mut self.face_1.corners {
    //         *p = p.rotate_y(angle);
    //     }
    //     for p in &mut self.face_1.markers {
    //         *p = p.rotate_y(angle);
    //     }
        
    //     for p in &mut self.face_2.corners {
    //         *p = p.rotate_y(angle);
    //     }
    //     for p in &mut self.face_2.markers {
    //         *p = p.rotate_y(angle);
    //     }
        
    //     for slice in &mut self.face_slices {
    //         for p in &mut slice.corners {
    //             *p = p.rotate_y(angle);
    //         }
    //         for p in &mut slice.markers {
    //             *p = p.rotate_y(angle);
    //         }
    //     }
    // }
}

impl Renderable for CubeSlice {
    fn get_visible_faces(&self) -> Vec<AnyFace> {
        let mut faces = vec![
            AnyFace::Face(self.face_1.clone()),
            AnyFace::Face(self.face_2.clone()),
        ];

        faces.extend(self.face_slices.iter()
            .cloned()
            .map(AnyFace::FaceSlice));

        faces.sort_by(|a, b| a.avg_z().partial_cmp(&b.avg_z()).unwrap());
        faces
    }

    fn dist(&self) -> f32 {
        let mut sum = 0.0;
        
        sum += self.face_1.avg_z();
        sum += self.face_2.avg_z();

        for fs in &self.face_slices {
            sum += fs.avg_z();
            sum += fs.avg_y() * TIEBREAKER_WEIGHT; // secret sauce
            sum += fs.avg_x() * TIEBREAKER_WEIGHT; // secret sauce
        }

        sum / 6.0
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::core::grid::GridFace;

    use super::*;

    fn point(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    #[test]
    fn rotate_x_90_degrees() {
        let mut slice = CubeSlice {
            global_cube_position: point(0.0, 0.0, 0.0),
            face_1: Face {
                corners: [point(1.0, 0.0, 0.0); 4],
                markers: vec![point(1.0, 0.0, 0.0)],
                grid_face: GridFace::empty(),
            },
            face_2: Face {
                corners: [point(0.0, 1.0, 0.0); 4],
                markers: vec![point(0.0, 1.0, 0.0)],
                grid_face: GridFace::empty(),
            },
            face_slices: [
                FaceSlice::new([point(0.0,0.0,1.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,0.0,1.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,0.0,1.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,0.0,1.0);4], [Color::Green;3]),
            ],
        };

        // punkt (0,1,0) obrócony o 90° wokół osi X powinien wylądować w (0,0,1)
        slice.rotate_x(std::f32::consts::FRAC_PI_2);
        let p = slice.face_2.corners[0];
        assert!((p.x - 0.0).abs() < 1e-6);
        assert!((p.y - 0.0).abs() < 1e-6);
        assert!((p.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rotate_y_90_degrees() {
        let mut slice = CubeSlice {
            global_cube_position: point(0.0, 0.0, 0.0),
            face_1: Face {
                corners: [point(1.0, 0.0, 0.0); 4],
                markers: vec![point(1.0, 0.0, 0.0)],
                grid_face: GridFace::empty(),
            },
            face_2: Face {
                corners: [point(0.0, 0.0, 1.0); 4],
                markers: vec![point(0.0, 0.0, 1.0)],
                grid_face: GridFace::empty(),
            },
            face_slices: [
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
            ],
        };

        // punkt (1,0,0) obrócony o 90° wokół osi Y powinien wylądować w (0,0,-1)
        slice.rotate_y(std::f32::consts::FRAC_PI_2);
        let p = slice.face_1.corners[0];
        assert!((p.x - 0.0).abs() < 1e-6);
        assert!((p.y - 0.0).abs() < 1e-6);
        assert!((p.z + 1.0).abs() < 1e-6);
    }

    #[test]
    fn cube_slice_rotation_random_angles() {
        let mut slice = CubeSlice {
            global_cube_position: point(0.0, 0.0, 0.0),
            face_1: Face {
                corners: [point(1.0, 0.0, 0.0); 4],
                markers: vec![point(1.0, 0.0, 0.0)],
                grid_face: GridFace::empty(),
            },
            face_2: Face {
                corners: [point(0.0, 0.0, 1.0); 4],
                markers: vec![point(0.0, 0.0, 1.0)],
                grid_face: GridFace::empty(),
            },
            face_slices: [
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
                FaceSlice::new([point(0.0,1.0,0.0);4], [Color::Green;3]),
            ],
        };

        // punkty przed obrotem
        let before: Vec<Point3D> = slice
            .face_1
            .corners
            .iter()
            .chain(slice.face_2.corners.iter())
            .cloned()
            .collect();

        // obracamy wokół X i Y
        slice.rotate_x(0.37);
        slice.rotate_y(-0.83);

        // punkty po obrocie
        let after: Vec<Point3D> = slice
            .face_1
            .corners
            .iter()
            .chain(slice.face_2.corners.iter())
            .cloned()
            .collect();

        // sprawdzamy odległości od środka slice’a
        let center = slice.global_cube_position;
        let dist_before: Vec<f32> = before
            .iter()
            .map(|p| ((p.x - center.x).powi(2) + (p.y - center.y).powi(2) + (p.z - center.z).powi(2)).sqrt())
            .collect();

        let dist_after: Vec<f32> = after
            .iter()
            .map(|p| ((p.x - center.x).powi(2) + (p.y - center.y).powi(2) + (p.z - center.z).powi(2)).sqrt())
            .collect();

        for (d1, d2) in dist_before.iter().zip(dist_after.iter()) {
            assert!(
                (d1 - d2).abs() < 1e-5,
                "Slice zdeformował się po obrocie! {} != {}", d1, d2
            );
        }
    }
}
