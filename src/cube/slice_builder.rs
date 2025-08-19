use crate::{
    utils::{
        cube_utils::{Axis, Color}
    },
    cube::{
        slice::{CubeSlice, CubeSliceOrder},
        cube::{Cube, Face},
        core::{
            grid::{Grid, GridFace, GridSide, NeighborSlice}
        }
    }
};

pub struct CubeSliceBuilder<'a> {
    cube: &'a Cube,
    split_faces: (GridSide, GridSide),
    idx_1: (usize, usize),
    idx_2: (usize, usize),
    idx_3: (usize, usize),
    idx_4: (usize, usize),
    face_1: GridSide,
    face_2: GridSide,
}

impl <'a> CubeSliceBuilder<'a> {
    pub fn create_cube_slices(cube: &Cube, grid: &Grid, axis: &Axis) -> [CubeSlice; 3] {
        let builder: CubeSliceBuilder = match axis {
            Axis::X => CubeSliceBuilder {
                cube,
                split_faces: (
                    GridSide::Top,
                    GridSide::Bottom
                ),
                idx_1: (13, 1),
                idx_2: (13, 1),
                idx_3: (2, 14),
                idx_4: (2, 14),
                face_1: GridSide::Left,
                face_2: GridSide::Right,
            },
            Axis::Y => CubeSliceBuilder {
                cube,
                split_faces: (
                    GridSide::Back,
                    GridSide::Front
                ),
                idx_1: (4, 7),
                idx_2: (4, 7),
                idx_3: (11, 8),
                idx_4: (11, 8),
                face_1: GridSide::Top,
                face_2: GridSide::Bottom,
            },
            Axis::Z => CubeSliceBuilder {
                cube,
                split_faces: (
                    GridSide::Top,
                    GridSide::Bottom
                ),
                idx_1: (11, 8),
                idx_2: (4, 7),
                idx_3: (4, 7),
                idx_4: (11, 8),
                face_1: GridSide::Front,
                face_2: GridSide::Back,
            },
        };

        builder.build_cube_slices(grid)
    }

    fn build_cube_slices(self, grid: &Grid) -> [CubeSlice; 3] {
        let sf_0_idx = self.split_faces.0.idx();
        let sf_2_idx = self.split_faces.1.idx();

        let f_1_idx = self.face_1.idx();
        let f_2_idx = self.face_2.idx();

        let axis = &self.face_1.axis();

        let mut last_corners = [
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.0).unwrap().clone(),
            self.cube.faces[sf_0_idx].markers.get(self.idx_3.1).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.0).unwrap().clone(),
            self.cube.faces[sf_2_idx].markers.get(self.idx_4.1).unwrap().clone(),
        ];

        if let Axis::Y = axis {
            last_corners.rotate_right(2);
        }

        let neighbors_1 = grid.get_neighbors(self.face_1);
        let neighbors_1_colors: Vec<[Color; 3]> = self.get_slices_colors(neighbors_1, grid);
        
        let middles = grid.get_neighbors(GridSide::middle_layer_from_axis(axis));
        let middles_colors: Vec<[Color; 3]> = self.get_slices_colors(middles, grid);

        let neighbors_2 = grid.get_neighbors(self.face_2);
        let neighbors_2_colors: Vec<[Color; 3]> = self.get_slices_colors(neighbors_2, grid);

        [
            CubeSlice::new(
                self.cube.faces[f_1_idx].clone(),
                Face::new(
                    [
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.0).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.1).unwrap().clone(),
                    ], GridFace::empty()
                ),
                neighbors_1_colors,
                axis,
                CubeSliceOrder::FIRST
            ),
            CubeSlice::new(
                Face::new(
                    [
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.1).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_1.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_2.0).unwrap().clone(),
                    ], GridFace::empty()
                ),
                Face::new([
                        self.cube.faces[sf_0_idx].markers.get(self.idx_3.1).unwrap().clone(),
                        self.cube.faces[sf_0_idx].markers.get(self.idx_3.0).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_4.1).unwrap().clone(),
                        self.cube.faces[sf_2_idx].markers.get(self.idx_4.0).unwrap().clone(),
                    ], GridFace::empty()
                ),
                middles_colors,
                axis,
                CubeSliceOrder::MIDDLE
            ),
            CubeSlice::new(
                Face::new(last_corners, GridFace::empty()),
                self.cube.faces[f_2_idx].clone(),
                neighbors_2_colors,
                axis,
                CubeSliceOrder::LAST
            ),
        ]
    }

    fn get_slices_colors(&self, slices: [NeighborSlice; 4], grid: &Grid) -> Vec<[Color; 3]> {
        slices.iter()
            .map(|ns| ns.read_from(grid))
            .collect()
    }
}