use super::cube::Color;

const PRINT_CHAR: &str = "██";
const ANSI_RESET: &str = "\x1b[0m";

pub enum MoveDirection {
    Clockwise,
    CounterClockwise,
    Double,
}

#[derive(Debug, Clone, Copy)]
pub enum GridSide {
    TOP,
    FRONT,
    BOTTOM,
    LEFT,
    RIGHT,
    BACK,
}

impl GridSide {
    pub fn idx(&self) -> usize {
        match self {
            GridSide::TOP => 0,
            GridSide::LEFT => 1,
            GridSide::FRONT => 2,
            GridSide::RIGHT => 3,
            GridSide::BACK => 4,
            GridSide::BOTTOM => 5,
        }
    }
}

enum SliceType {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

struct NeighborSlice {
    side: GridSide,
    slice_type: SliceType,
}

impl NeighborSlice {
    fn read_from(&self, grid: &Grid) -> [Color; 3] {
        let face = &grid.faces[self.side.idx()];
        match self.slice_type {
            SliceType::TOP => {
                [
                    face.grid[0][0],
                    face.grid[0][1],
                    face.grid[0][2],
                ]
            },
            SliceType::BOTTOM => {
                [
                    face.grid[2][2],
                    face.grid[2][1],
                    face.grid[2][0],
                ]
            },
            SliceType::LEFT => {
                [
                    face.grid[2][0],
                    face.grid[1][0],
                    face.grid[0][0],
                ]
            },
            SliceType::RIGHT => {
                [
                    face.grid[0][2],
                    face.grid[1][2],
                    face.grid[2][2],
                ]
            },
        }
    }

    fn write_to(&self, grid: &mut Grid, colors: [Color; 3]) {
        let face = &mut grid.faces[self.side.idx()];
        match self.slice_type {
            SliceType::TOP => {
                face.grid[0][0] = colors[0];
                face.grid[0][1] = colors[1];
                face.grid[0][2] = colors[2];
            },
            SliceType::BOTTOM => {
                face.grid[2][2] = colors[0];
                face.grid[2][1] = colors[1];
                face.grid[2][0] = colors[2];
            },
            SliceType::LEFT => {
                face.grid[2][0] = colors[0];
                face.grid[1][0] = colors[1];
                face.grid[0][0] = colors[2];
            },
            SliceType::RIGHT => {
                face.grid[0][2] = colors[0];
                face.grid[1][2] = colors[1];
                face.grid[2][2] = colors[2];
            },
        }
    }
}

#[derive(Clone, Copy)]
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
        GridFace {
            grid: [[Color::Gray; 3]; 3],
        }
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

    pub fn apply_move(&mut self, mv: &str) -> Result<(), String> {
        let (side_char, suffix) = mv.split_at(1);
        let side = match side_char {
            "R" => GridSide::RIGHT,
            "L" => GridSide::LEFT,
            "U" => GridSide::TOP,
            "D" => GridSide::BOTTOM,
            "F" => GridSide::FRONT,
            "B" => GridSide::BACK,
            _ => return Err(format!("Incorrect move '{}'", mv)),
        };
        let direction = match suffix {
            "" => Ok(MoveDirection::Clockwise),
            "'" => Ok(MoveDirection::CounterClockwise),
            "2" => Ok(MoveDirection::Double),
            _ => Err(format!("Incorrect move '{}'", mv)),
        }?;

        self.move_face(side, direction);

        Ok(())
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

    fn rotate_buffers(buffers: &mut Vec<[Color; 3]>, direction: MoveDirection) {
        match direction {
            MoveDirection::Clockwise => buffers.rotate_right(1),
            MoveDirection::CounterClockwise => buffers.rotate_left(1),
            MoveDirection::Double => buffers.rotate_right(2),
        }
    }

    pub fn move_face(&mut self, side: GridSide, direction: MoveDirection) {
        let idx = side.idx();
        self.faces[idx].rotate(&direction);
        let neighbors = self.get_neighbors(side);
        let mut buffers: Vec<[Color; 3]> = neighbors.iter()
            .map(|ns| {
                    ns.read_from(self)
                }
            )
            .collect();

        Grid::rotate_buffers(&mut buffers, direction);

        for (slice, colors) in neighbors.into_iter().zip(buffers) {
            slice.write_to(self, colors);
        }        
    }

    fn get_neighbors(&self, side: GridSide) -> [NeighborSlice; 4] {
        match side {
            GridSide::TOP => [
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::FRONT},
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::LEFT},
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::BACK},
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::RIGHT},
            ],
            GridSide::FRONT => [
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::BOTTOM},
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::LEFT},
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::TOP},
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::RIGHT},
            ],
            GridSide::BOTTOM => [
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::FRONT},
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::RIGHT},
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::BACK},
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::LEFT},
            ],
            GridSide::LEFT => [
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::TOP},
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::FRONT},
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::BOTTOM},
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::BACK},
            ],
            GridSide::RIGHT => [
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::TOP},
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::BACK},
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::BOTTOM},
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::FRONT},
            ],
            GridSide::BACK => [
                NeighborSlice {slice_type: SliceType::TOP, side: GridSide::TOP},
                NeighborSlice {slice_type: SliceType::LEFT, side: GridSide::LEFT},
                NeighborSlice {slice_type: SliceType::BOTTOM, side: GridSide::BOTTOM},
                NeighborSlice {slice_type: SliceType::RIGHT, side: GridSide::RIGHT},
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::{Color};
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
        assert_whole_color(grid, GridSide::TOP, Color::White) 
            && assert_whole_color(grid, GridSide::LEFT, Color::Orange)
            && assert_whole_color(grid, GridSide::FRONT, Color::Green)
            && assert_whole_color(grid, GridSide::RIGHT, Color::Red)
            && assert_whole_color(grid, GridSide::BACK, Color::Blue)
            && assert_whole_color(grid, GridSide::BOTTOM, Color::Yellow)
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
        fn new_custom_face(grid: [[Color; 3]; 3]) -> GridFace {
            GridFace { grid }
        }

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
        grid.move_face(GridSide::RIGHT, MoveDirection::Clockwise);
        
        assert!(assert_whole_color(&grid, GridSide::RIGHT, Color::Red));
        assert!(assert_right_color(&grid, GridSide::FRONT, Color::Yellow));
        assert!(assert_right_color(&grid, GridSide::TOP, Color::Green));
        assert!(assert_left_color(&grid, GridSide::BACK, Color::White));
        assert!(assert_right_color(&grid, GridSide::BOTTOM, Color::Blue));

        grid.move_face(GridSide::RIGHT, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_left() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::LEFT, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::LEFT, Color::Orange));
        assert!(assert_left_color(&grid, GridSide::FRONT, Color::White));
        assert!(assert_left_color(&grid, GridSide::TOP, Color::Blue));
        assert!(assert_right_color(&grid, GridSide::BACK, Color::Yellow));
        assert!(assert_left_color(&grid, GridSide::BOTTOM, Color::Green));

        grid.move_face(GridSide::LEFT, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_front() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::FRONT, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::FRONT, Color::Green));
        assert!(assert_bottom_color(&grid, GridSide::TOP, Color::Orange));
        assert!(assert_left_color(&grid, GridSide::RIGHT, Color::White));
        assert!(assert_top_color(&grid, GridSide::BOTTOM, Color::Red));
        assert!(assert_right_color(&grid, GridSide::LEFT, Color::Yellow));

        grid.move_face(GridSide::FRONT, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_top() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::TOP, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::TOP, Color::White));
        assert!(assert_top_color(&grid, GridSide::FRONT, Color::Red));
        assert!(assert_top_color(&grid, GridSide::LEFT, Color::Green));
        assert!(assert_top_color(&grid, GridSide::RIGHT, Color::Blue));
        assert!(assert_top_color(&grid, GridSide::BACK, Color::Orange));

        grid.move_face(GridSide::TOP, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_bottom() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::BOTTOM, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::BOTTOM, Color::Yellow));
        assert!(assert_bottom_color(&grid, GridSide::FRONT, Color::Orange));
        assert!(assert_bottom_color(&grid, GridSide::LEFT, Color::Blue));
        assert!(assert_bottom_color(&grid, GridSide::RIGHT, Color::Green));
        assert!(assert_bottom_color(&grid, GridSide::BACK, Color::Red));

        grid.move_face(GridSide::BOTTOM, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_move_back() {
        let mut grid = create_solved_grid();
        grid.move_face(GridSide::BACK, MoveDirection::Clockwise);

        assert!(assert_whole_color(&grid, GridSide::BACK, Color::Blue));
        assert!(assert_top_color(&grid, GridSide::TOP, Color::Red));
        assert!(assert_right_color(&grid, GridSide::RIGHT, Color::Yellow));
        assert!(assert_left_color(&grid, GridSide::LEFT, Color::White));
        assert!(assert_bottom_color(&grid, GridSide::BOTTOM, Color::Orange));

        grid.move_face(GridSide::BACK, MoveDirection::CounterClockwise);

        assert!(assert_solved_cube(&grid));
    }

    #[test]
    fn test_solving_mixed_cuve() {
        use GridSide::*;
        use MoveDirection::*;

        let mut grid = create_mixed_grid();

        let moves = vec![
            (BOTTOM, Clockwise),
            (LEFT, CounterClockwise),
            (FRONT, Clockwise),
            (RIGHT, CounterClockwise),
            
            (BOTTOM, Clockwise),
            (RIGHT, Clockwise),
            (RIGHT, Clockwise),
            (LEFT, Clockwise),

            (FRONT, Clockwise),
            (LEFT, CounterClockwise),
            (RIGHT, Clockwise),
            (RIGHT, Clockwise),

            (FRONT, CounterClockwise),
            (TOP, Clockwise),
            (TOP, Clockwise),
            (BACK, CounterClockwise),

            (RIGHT, Clockwise),
            (RIGHT, Clockwise),
            (BACK, Clockwise),
            (TOP, Clockwise),
            (TOP, Clockwise),

            (FRONT, CounterClockwise),
            (LEFT, Clockwise),
            (LEFT, Clockwise),

            (TOP, Clockwise),
            (TOP, Clockwise),
            (BACK, Clockwise),
        ];

        for (side, direction) in moves {
            grid.move_face(side, direction);
        }

        assert!(assert_solved_cube(&grid))
    }
}
