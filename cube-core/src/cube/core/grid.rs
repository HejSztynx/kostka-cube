use crate::{
    utils::{
        cube_utils::{Axis, Color},
    },
    cube::{
        slice::{CubeMove, CubeSliceOrder}
    }
};

const PRINT_CHAR: &str = "██";
const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub enum MoveDirection {
    Clockwise,
    CounterClockwise,
    Double,
}

impl MoveDirection {
    pub fn flip(self) -> MoveDirection {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise,
            Self::Double => Self::Double,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridSide {
    Top,
    Front,
    Bottom,
    Left,
    Right,
    Back,
    MiddleX,
    MiddleY,
    MiddleZ,
}

impl GridSide {
    pub fn idx(&self) -> usize {
        match self {
            GridSide::Top => 0,
            GridSide::Left => 1,
            GridSide::Front => 2,
            GridSide::Right => 3,
            GridSide::Back => 4,
            GridSide::Bottom => 5,
            _ => panic!()
        }
    }

    pub fn from_idx(idx: usize) -> GridSide {
        match idx {
            0 => GridSide::Top,
            1 => GridSide::Left,
            2 => GridSide::Front,
            3 => GridSide::Right,
            4 => GridSide::Back,
            5 => GridSide::Bottom,
            _ => panic!(),
        }
    }

    pub fn middle_layer_from_axis(axis: &Axis) -> GridSide {
        match axis {
            Axis::X => GridSide::MiddleX,
            Axis::Y => GridSide::MiddleY,
            Axis::Z => GridSide::MiddleZ
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            GridSide::Top => Axis::Y,
            GridSide::Left => Axis::X,
            GridSide::Front => Axis::Z,
            GridSide::Right => Axis::X,
            GridSide::Back => Axis::Z,
            GridSide::Bottom => Axis::Y,
            GridSide::MiddleX => Axis::X,
            GridSide::MiddleY => Axis::Y,
            GridSide::MiddleZ => Axis::Z,
        }
    }

    pub fn order(&self) -> CubeSliceOrder {
        match self {
            GridSide::Top => CubeSliceOrder::FIRST,
            GridSide::Left => CubeSliceOrder::FIRST,
            GridSide::Front => CubeSliceOrder::FIRST,
            GridSide::Right => CubeSliceOrder::LAST,
            GridSide::Back => CubeSliceOrder::LAST,
            GridSide::Bottom => CubeSliceOrder::LAST,
            GridSide::MiddleX => CubeSliceOrder::MIDDLE,
            GridSide::MiddleY => CubeSliceOrder::MIDDLE,
            GridSide::MiddleZ => CubeSliceOrder::MIDDLE,
        }
    }

    pub fn is_middle(&self) -> bool {
        match self {
            GridSide::MiddleX => true,
            GridSide::MiddleY => true,
            GridSide::MiddleZ => true,
            _ => false
        }
    }

    pub fn middle_layer_adjacent(self) -> GridSide {
        match self {
            GridSide::MiddleX => GridSide::Left,
            GridSide::MiddleY => GridSide::Bottom,
            GridSide::MiddleZ => GridSide::Front,
            _ => self
        }
    }
}

enum SliceType {
    Top,
    Bottom,
    Left,
    Right,
    Vertical,
    VerticalFlipped,
    Horizontal,
    HorizontalFlipped,
}

pub struct NeighborSlice {
    side: GridSide,
    slice_type: SliceType,
}

impl NeighborSlice {
    pub fn read_from(&self, grid: &Grid) -> [Color; 3] {
        let face = &grid.faces[self.side.idx()];
        match self.slice_type {
            SliceType::Top => {
                [
                    face.grid[0][0],
                    face.grid[0][1],
                    face.grid[0][2],
                ]
            },
            SliceType::Bottom => {
                [
                    face.grid[2][2],
                    face.grid[2][1],
                    face.grid[2][0],
                ]
            },
            SliceType::Left => {
                [
                    face.grid[2][0],
                    face.grid[1][0],
                    face.grid[0][0],
                ]
            },
            SliceType::Right => {
                [
                    face.grid[0][2],
                    face.grid[1][2],
                    face.grid[2][2],
                ]
            },
            SliceType::Vertical => {
                [
                    face.grid[2][1],
                    face.grid[1][1],
                    face.grid[0][1],
                ]
            },
            SliceType::VerticalFlipped => {
                [
                    face.grid[0][1],
                    face.grid[1][1],
                    face.grid[2][1],
                ]
            },
            SliceType::Horizontal => {
                [
                    face.grid[1][0],
                    face.grid[1][1],
                    face.grid[1][2],
                ]
            },
            SliceType::HorizontalFlipped => {
                [
                    face.grid[1][2],
                    face.grid[1][1],
                    face.grid[1][0],
                ]
            },
        }
    }

    fn write_to(&self, grid: &mut Grid, colors: [Color; 3]) {
        let face = &mut grid.faces[self.side.idx()];
        match self.slice_type {
            SliceType::Top => {
                face.grid[0][0] = colors[0];
                face.grid[0][1] = colors[1];
                face.grid[0][2] = colors[2];
            },
            SliceType::Bottom => {
                face.grid[2][2] = colors[0];
                face.grid[2][1] = colors[1];
                face.grid[2][0] = colors[2];
            },
            SliceType::Left => {
                face.grid[2][0] = colors[0];
                face.grid[1][0] = colors[1];
                face.grid[0][0] = colors[2];
            },
            SliceType::Right => {
                face.grid[0][2] = colors[0];
                face.grid[1][2] = colors[1];
                face.grid[2][2] = colors[2];
            },
            SliceType::Vertical => {
                face.grid[2][1] = colors[0];
                face.grid[1][1] = colors[1];
                face.grid[0][1] = colors[2];
            },
            SliceType::VerticalFlipped => {
                face.grid[2][1] = colors[2];
                face.grid[1][1] = colors[1];
                face.grid[0][1] = colors[0];
            },
            SliceType::Horizontal => {
                face.grid[1][0] = colors[0];
                face.grid[1][1] = colors[1];
                face.grid[1][2] = colors[2];
            },
            SliceType::HorizontalFlipped => {
                face.grid[1][0] = colors[2];
                face.grid[1][1] = colors[1];
                face.grid[1][2] = colors[0];
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridFace {
    pub grid: [[Color; 3]; 3],
}

impl GridFace {
    pub fn new(color: Color) -> GridFace {
        GridFace {
            grid: [[color; 3]; 3],
        }
    }

    pub fn empty() -> GridFace {
        Self::new(Color::Gray)
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for color in row {
                print!("{}{}{}", color.to_ansi(), PRINT_CHAR, ANSI_RESET);
            }
            println!();
        }
    }

    pub fn print_row(&self, idx: usize) {
        for color in self.grid[idx] {
            print!("{}{}{}", color.to_ansi(), PRINT_CHAR, ANSI_RESET);
        }
    }

    pub fn rotate(&mut self, direction: &MoveDirection) {
        match direction {
            MoveDirection::Clockwise => self.rotate_clockwise(),
            MoveDirection::CounterClockwise => self.rotate_counter_clockwise(),
            MoveDirection::Double => {
                self.rotate_clockwise();
                self.rotate_clockwise();
            }
        }
    }

    pub fn rotate_clockwise(&mut self) {
        // rotate corners
        let tmp = self.grid[0][0];
        self.grid[0][0] = self.grid[2][0];
        self.grid[2][0] = self.grid[2][2];
        self.grid[2][2] = self.grid[0][2];
        self.grid[0][2] = tmp;

        // rotate edges
        let tmp = self.grid[0][1];
        self.grid[0][1] = self.grid[1][0];
        self.grid[1][0] = self.grid[2][1];
        self.grid[2][1] = self.grid[1][2];
        self.grid[1][2] = tmp;
    }

    pub fn rotate_counter_clockwise(&mut self) {
        // rotate corners
        let tmp = self.grid[0][0];
        self.grid[0][0] = self.grid[0][2];
        self.grid[0][2] = self.grid[2][2];
        self.grid[2][2] = self.grid[2][0];
        self.grid[2][0] = tmp;

        // rotate edges
        let tmp = self.grid[0][1];
        self.grid[0][1] = self.grid[1][2];
        self.grid[1][2] = self.grid[2][1];
        self.grid[2][1] = self.grid[1][0];
        self.grid[1][0] = tmp;
    }
}

pub struct Grid {
    pub faces: [GridFace; 6],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            faces: [
                GridFace::new(Color::White),
                GridFace::new(Color::Orange),
                GridFace::new(Color::Green),
                GridFace::new(Color::Red),
                GridFace::new(Color::Blue),
                GridFace::new(Color::Yellow),
            ]
        }
    }

    pub fn apply_move(&mut self, mv: CubeMove) {
        self.move_face(mv.grid_side, mv.direction);
    }

    pub fn print(&self) {
        fn print_blank_row() {
            for _ in 0..3 {
                print!("{}{}{}", Color::Gray.to_ansi(), PRINT_CHAR, ANSI_RESET);
            }
        }

        for row in 0..3 {
            print_blank_row();
            self.faces[0].print_row(row);
            print_blank_row();
            print_blank_row();
            println!();
        }

        for row in 0..3 {
            for face in 1..5 {
                self.faces[face].print_row(row);
            }
            println!();
        }

        for row in 0..3 {
            print_blank_row();
            self.faces[5].print_row(row);
            print_blank_row();
            print_blank_row();
            println!();
        }

        print!("\n\n\n");
    }

    fn rotate_buffers(buffers: &mut Vec<[Color; 3]>, grid_side: &GridSide, direction: MoveDirection) {
        match grid_side {
            GridSide::Left 
                | GridSide::Top
                | GridSide::Front
                | GridSide::MiddleX
                | GridSide::MiddleZ => match direction {
                MoveDirection::Clockwise => buffers.rotate_right(1),
                MoveDirection::CounterClockwise => buffers.rotate_left(1),
                MoveDirection::Double => buffers.rotate_right(2),
            }
            _ => match direction {
                MoveDirection::Clockwise => buffers.rotate_left(1),
                MoveDirection::CounterClockwise => buffers.rotate_right(1),
                MoveDirection::Double => buffers.rotate_right(2),
            }
        };
        
    }

    pub fn move_face(&mut self, side: GridSide, direction: MoveDirection) {
        if !side.is_middle() {
            let idx = side.idx();
            self.faces[idx].rotate(&direction);
        }

        let neighbors = self.get_neighbors(side);
        let mut buffers: Vec<[Color; 3]> = neighbors.iter()
            .map(|ns| {
                    ns.read_from(self)
                }
            )
            .collect();

        Grid::rotate_buffers(&mut buffers, &side, direction);

        for (slice, colors) in neighbors.into_iter().zip(buffers) {
            slice.write_to(self, colors);
        }        
    }

    pub fn get_neighbors(&self, side: GridSide) -> [NeighborSlice; 4] {
        match side {
            GridSide::Top => [
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Back},
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Left},
            ],
            GridSide::Front => [
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Left},
            ],
            GridSide::Bottom => [
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Back},
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Left},
            ],
            GridSide::Left => [
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Back},
            ],
            GridSide::Right => [
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Back},
            ],
            GridSide::Back => [
                NeighborSlice {slice_type: SliceType::Top, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Right, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Bottom, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::Left, side: GridSide::Left},
            ],
            GridSide::MiddleX => [
                NeighborSlice {slice_type: SliceType::Vertical, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Vertical, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Vertical, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::VerticalFlipped, side: GridSide::Back},
            ],
            GridSide::MiddleY => [
                NeighborSlice {slice_type: SliceType::Horizontal, side: GridSide::Back},
                NeighborSlice {slice_type: SliceType::Horizontal, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Horizontal, side: GridSide::Front},
                NeighborSlice {slice_type: SliceType::Horizontal, side: GridSide::Left},
            ],
            GridSide::MiddleZ => [
                NeighborSlice {slice_type: SliceType::HorizontalFlipped, side: GridSide::Top},
                NeighborSlice {slice_type: SliceType::Vertical, side: GridSide::Right},
                NeighborSlice {slice_type: SliceType::Horizontal, side: GridSide::Bottom},
                NeighborSlice {slice_type: SliceType::VerticalFlipped, side: GridSide::Left},
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::cube_utils::Color;
    use Color::*;

    fn assert_whole_color(grid: &Grid, side: GridSide, color: Color) -> bool {
        let face = &grid.faces[side.idx()];
        for row in face.grid {
            for c in row {
                if c != color {
                    return false;
                }
            }
        };
        true
    }

    fn assert_right_color(grid: &Grid, side: GridSide, color: Color) -> bool {
        let face_grid = &grid.faces[side.idx()].grid;
        face_grid[0][2] == color && face_grid[1][2] == color && face_grid[2][2] == color
    }

    fn assert_left_color(grid: &Grid, side: GridSide, color: Color) -> bool {
        let face_grid = &grid.faces[side.idx()].grid;
        face_grid[0][0] == color && face_grid[1][0] == color && face_grid[2][0] == color
    }

    fn assert_top_color(grid: &Grid, side: GridSide, color: Color) -> bool {
        let face_grid = &grid.faces[side.idx()].grid;
        face_grid[0][0] == color && face_grid[0][1] == color && face_grid[0][2] == color
    }

    fn assert_bottom_color(grid: &Grid, side: GridSide, color: Color) -> bool {
        let face_grid = &grid.faces[side.idx()].grid;
        face_grid[2][0] == color && face_grid[2][1] == color && face_grid[2][2] == color
    }

    fn assert_solved_cube(grid: &Grid) -> bool {
        assert_whole_color(grid, GridSide::Top, Color::White) 
            && assert_whole_color(grid, GridSide::Left, Color::Orange)
            && assert_whole_color(grid, GridSide::Front, Color::Green)
            && assert_whole_color(grid, GridSide::Right, Color::Red)
            && assert_whole_color(grid, GridSide::Back, Color::Blue)
            && assert_whole_color(grid, GridSide::Bottom, Color::Yellow)
    }

    fn new_custom_face(grid: [[Color; 3]; 3]) -> GridFace {
        GridFace { grid }
    }

    fn create_solved_grid() -> Grid {
        Grid {
            faces: [
                GridFace::new(White),   // TOP
                GridFace::new(Orange),  // LEFT
                GridFace::new(Green),   // FRONT
                GridFace::new(Red),     // RIGHT
                GridFace::new(Blue),    // BACK
                GridFace::new(Yellow),  // BOTTOM
            ],
        }
    }

    fn create_mixed_grid() -> Grid {
        Grid {
            faces: [
                // TOP
                new_custom_face([
                    [Red, White, Green],
                    [Green, White, Blue],
                    [White, Blue, Blue],
                ]),
                // LEFT
                new_custom_face([
                    [Blue, Orange, Red],
                    [Yellow, Orange, Yellow],
                    [Green, Green, Orange],
                ]),
                // FRONT
                new_custom_face([
                    [Blue, Red, Orange],
                    [Green, Green, Orange],
                    [Yellow, Yellow, Green],
                ]),
                // RIGHT
                new_custom_face([
                    [White, Orange, Orange],
                    [Yellow, Red, Orange],
                    [Red, Red, Green],
                ]),
                // BACK
                new_custom_face([
                    [Yellow, Green, Yellow],
                    [White, Blue, Red],
                    [White, White, White],
                ]),
                // BOTTOM
                new_custom_face([
                    [Blue, Blue, Yellow],
                    [Red, Yellow, White],
                    [Orange, Blue, Red],
                ]),
            ],
        }
    }

    #[test]
    fn test_move_right() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Right, MoveDirection::Clockwise);
        
        assert!(assert_whole_color(&grid, GridSide::Right, Color::Red));
        assert!(assert_right_color(&grid, GridSide::Front, Color::Yellow));
        assert!(assert_right_color(&grid, GridSide::Top, Color::Green));
        assert!(assert_left_color(&grid, GridSide::Back, Color::White));
        assert!(assert_right_color(&grid, GridSide::Bottom, Color::Blue));

        grid.move_face(GridSide::Right, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_left() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Left, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::Left, Color::Orange));
        assert!(assert_left_color(&grid, GridSide::Front, Color::White));
        assert!(assert_left_color(&grid, GridSide::Top, Color::Blue));
        assert!(assert_right_color(&grid, GridSide::Back, Color::Yellow));
        assert!(assert_left_color(&grid, GridSide::Bottom, Color::Green));

        grid.move_face(GridSide::Left, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_front() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Front, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::Front, Color::Green));
        assert!(assert_bottom_color(&grid, GridSide::Top, Color::Orange));
        assert!(assert_left_color(&grid, GridSide::Right, Color::White));
        assert!(assert_top_color(&grid, GridSide::Bottom, Color::Red));
        assert!(assert_right_color(&grid, GridSide::Left, Color::Yellow));

        grid.move_face(GridSide::Front, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_top() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Top, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::Top, Color::White));
        assert!(assert_top_color(&grid, GridSide::Front, Color::Red));
        assert!(assert_top_color(&grid, GridSide::Left, Color::Green));
        assert!(assert_top_color(&grid, GridSide::Right, Color::Blue));
        assert!(assert_top_color(&grid, GridSide::Back, Color::Orange));

        grid.move_face(GridSide::Top, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_bottom() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Bottom, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::Bottom, Color::Yellow));
        assert!(assert_bottom_color(&grid, GridSide::Front, Color::Orange));
        assert!(assert_bottom_color(&grid, GridSide::Left, Color::Blue));
        assert!(assert_bottom_color(&grid, GridSide::Right, Color::Green));
        assert!(assert_bottom_color(&grid, GridSide::Back, Color::Red));

        grid.move_face(GridSide::Bottom, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_back() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::Back, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::Back, Color::Blue));
        assert!(assert_top_color(&grid, GridSide::Top, Color::Red));
        assert!(assert_right_color(&grid, GridSide::Right, Color::Yellow));
        assert!(assert_left_color(&grid, GridSide::Left, Color::White));
        assert!(assert_bottom_color(&grid, GridSide::Bottom, Color::Orange));

        grid.move_face(GridSide::Back, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_middle_move_m() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Top, Clockwise);
        grid.move_face(Bottom, Clockwise);
        grid.move_face(Front, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Top, Clockwise);
        
        grid.move_face(MiddleX, Clockwise);

        let front = &grid.faces[Front.idx()];
        assert_eq!(front, &new_custom_face([
                    [White, White, White],
                    [Orange, White, Yellow],
                    [Orange, Red, Yellow],
                ]));

        let top = &grid.faces[Top.idx()];
        assert_eq!(top, &new_custom_face([
                    [Blue, Red, White],
                    [Orange, Blue, White],
                    [Red, Green, Red],
                ]));

        let back = &grid.faces[Back.idx()];
        assert_eq!(back, &new_custom_face([
                    [Green, Yellow, Yellow],
                    [White, Yellow, Blue],
                    [White, Red, Red],
                ]));

        let bottom = &grid.faces[Bottom.idx()];
        assert_eq!(bottom, &new_custom_face([
                    [Green, White, Red],
                    [Yellow, Green, Blue],
                    [Yellow, Green, Orange],
                ]));
    }

    #[test]
    fn test_middle_move_m_counter_clockwise() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Top, Clockwise);
        grid.move_face(Bottom, Clockwise);
        grid.move_face(Front, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Top, Clockwise);
        
        grid.move_face(MiddleX, CounterClockwise);

        let front = &grid.faces[Front.idx()];
        assert_eq!(front, &new_custom_face([
                    [White, Red, White],
                    [Orange, Yellow, Yellow],
                    [Orange, Yellow, Yellow],
                ]));

        let top = &grid.faces[Top.idx()];
        assert_eq!(top, &new_custom_face([
                    [Blue, White, White],
                    [Orange, Green, White],
                    [Red, Green, Red],
                ]));

        let back = &grid.faces[Back.idx()];
        assert_eq!(back, &new_custom_face([
                    [Green, Red, Yellow],
                    [White, White, Blue],
                    [White, White, Red],
                ]));

        let bottom = &grid.faces[Bottom.idx()];
        assert_eq!(bottom, &new_custom_face([
                    [Green, Red, Red],
                    [Yellow, Blue, Blue],
                    [Yellow, Green, Orange],
                ]));
    }

    #[test]
    fn test_middle_move_s() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Top, Clockwise);
        grid.move_face(Bottom, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Back, Clockwise);
        grid.move_face(Top, Clockwise);

        grid.move_face(MiddleZ, Clockwise);

        let top = &grid.faces[Top.idx()];
        assert_eq!(top, &new_custom_face([
                    [White, White, Blue],
                    [Blue, Orange, Red],
                    [Orange, Green, Blue],
                ]));

        let right = &grid.faces[Right.idx()];
        assert_eq!(right, &new_custom_face([
                    [White, White, White],
                    [Green, White, Yellow],
                    [Green, Blue, Yellow],
                ]));
                
        let bottom = &grid.faces[Bottom.idx()];
        assert_eq!(bottom, &new_custom_face([
                    [Yellow, Yellow, Red],
                    [Red, Red, White],
                    [Green, Orange, Blue],
                ]));

        let left = &grid.faces[Left.idx()];
        assert_eq!(left, &new_custom_face([
                    [Red, Yellow, Yellow],
                    [White, Yellow, Orange],
                    [White, Blue, Blue],
                ]));

    }

    #[test]
    fn test_middle_move_s_counter_clockwise() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Top, Clockwise);
        grid.move_face(Bottom, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Back, Clockwise);
        grid.move_face(Top, Clockwise);

        grid.move_face(MiddleZ, CounterClockwise);

        let top = &grid.faces[Top.idx()];
        assert_eq!(top, &new_custom_face([
                    [White, White, Blue],
                    [White, Red, Red],
                    [Orange, Green, Blue],
                ]));

        let right = &grid.faces[Right.idx()];
        assert_eq!(right, &new_custom_face([
                    [White, Blue, White],
                    [Green, Yellow, Yellow],
                    [Green, Yellow, Yellow],
                ]));
                
        let bottom = &grid.faces[Bottom.idx()];
        assert_eq!(bottom, &new_custom_face([
                    [Yellow, Yellow, Red],
                    [Red, Orange, Blue],
                    [Green, Orange, Blue],
                ]));

        let left = &grid.faces[Left.idx()];
        assert_eq!(left, &new_custom_face([
                    [Red, Blue, Yellow],
                    [White, White, Orange],
                    [White, White, Blue],
                ]));

    }

    #[test]
    fn test_middle_move_e() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Front, Clockwise);
        grid.move_face(Back, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Top, Clockwise);
        grid.move_face(Front, Clockwise);

        grid.move_face(MiddleY, Clockwise);

        let front = &grid.faces[Front.idx()];
        assert_eq!(front, &new_custom_face([
                    [Green, Green, White],
                    [White, Orange, Red],
                    [Orange, Yellow, White],
                ]));

        let right = &grid.faces[Right.idx()];
        assert_eq!(right, &new_custom_face([
                    [Green, Blue, Blue],
                    [Green, Green, White],
                    [Green, Yellow, Yellow],
                ]));
                
        let back = &grid.faces[Back.idx()];
        assert_eq!(back, &new_custom_face([
                    [White, Orange, Yellow],
                    [Green, Red, Red],
                    [Red, Blue, Blue],
                ]));

        let left = &grid.faces[Left.idx()];
        assert_eq!(left, &new_custom_face([
                    [Green, Green, Red],
                    [White, Blue, Blue],
                    [White, Orange, Blue],
                ]));

    }

    #[test]
    fn test_middle_move_e_counter_clockwise() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_solved_grid();

        grid.move_face(Front, Clockwise);
        grid.move_face(Back, Clockwise);
        grid.move_face(Right, Clockwise);
        grid.move_face(Top, Clockwise);
        grid.move_face(Front, Clockwise);

        grid.move_face(MiddleY, CounterClockwise);

        let front = &grid.faces[Front.idx()];
        assert_eq!(front, &new_custom_face([
                    [Green, Green, White],
                    [Green, Red, Red],
                    [Orange, Yellow, White],
                ]));

        let right = &grid.faces[Right.idx()];
        assert_eq!(right, &new_custom_face([
                    [Green, Blue, Blue],
                    [White, Blue, Blue],
                    [Green, Yellow, Yellow],
                ]));
                
        let back = &grid.faces[Back.idx()];
        assert_eq!(back, &new_custom_face([
                    [White, Orange, Yellow],
                    [White, Orange, Red],
                    [Red, Blue, Blue],
                ]));

        let left = &grid.faces[Left.idx()];
        assert_eq!(left, &new_custom_face([
                    [Green, Green, Red],
                    [Green, Green, White],
                    [White, Orange, Blue],
                ]));

    }

    #[test]
    fn test_solving_mixed_cube() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_mixed_grid();

        let moves = vec![
            (Bottom, Clockwise),
            (Left, CounterClockwise),
            (Front, Clockwise),
            (Right, CounterClockwise),
            
            (Bottom, Clockwise),
            (Right, Clockwise),
            (Right, Clockwise),
            (Left, Clockwise),

            (Front, Clockwise),
            (Left, CounterClockwise),
            (Right, Clockwise),
            (Right, Clockwise),

            (Front, CounterClockwise),
            (Top, Clockwise),
            (Top, Clockwise),
            (Back, CounterClockwise),

            (Right, Clockwise),
            (Right, Clockwise),
            (Back, Clockwise),
            (Top, Clockwise),
            (Top, Clockwise),

            (Front, CounterClockwise),
            (Left, Clockwise),
            (Left, Clockwise),

            (Top, Clockwise),
            (Top, Clockwise),
            (Back, Clockwise),
        ];

        for (side, direction) in moves {
            grid.move_face(side, direction);
        }

        assert!(assert_solved_cube(&grid))
    }
}
