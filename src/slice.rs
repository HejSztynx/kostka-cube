use crate::{cube::{Color, Face}, geometry::Point3D, screen::{AnyFace, Renderable}};

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

        // for marker in markers.iter() {
            // println!("{:?}", marker);
        // }
        // println!("---------{:?}---------", colors[0]);
        // println!("---------KONIEC---------");
        FaceSlice { corners, markers, colors }
    }

    pub fn avg_z(&self) -> f32 {
        (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.0
    }
}

pub struct CubeSlice {
    pub face_1: Face,
    pub face_2: Face,
    pub face_slices: [FaceSlice; 4],
}

impl CubeSlice {
    pub fn new(face_1: Face, face_2: Face) -> CubeSlice {
        let face_slices = [
            FaceSlice::new([
                    face_1.corners[0],
                    face_2.corners[1],
                    face_2.corners[0],
                    face_1.corners[1],
                ],
                [Color::Blue; 3]
            ),
            FaceSlice::new([
                    face_2.corners[1],
                    face_1.corners[0],
                    face_1.corners[3],
                    face_2.corners[2],
                ],
                [Color::Red; 3]
            ),
            FaceSlice::new([
                    face_1.corners[2],
                    face_2.corners[3],
                    face_2.corners[2],
                    face_1.corners[3],
                ],
                [Color::Green; 3]
            ),
            FaceSlice::new([
                    face_1.corners[1],
                    face_2.corners[0],
                    face_2.corners[3],
                    face_1.corners[2],
                ],
                [Color::Orange; 3]
            ),
        ];

        print!("----------------------------------------------");

        CubeSlice { face_1, face_2, face_slices }
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
}