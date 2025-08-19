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

#[derive(Debug)]
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

#[derive(Debug)]
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

pub struct CubeSlice {
    pub face_1: Face,
    pub face_2: Face,
    pub face_slices: [FaceSlice; 4],
}

impl CubeSlice {
    pub fn new(face_1: Face, 
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

        CubeSlice { face_1, face_2, face_slices }
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