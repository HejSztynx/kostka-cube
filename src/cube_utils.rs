use crate::{
    grid::{GridSide, MoveDirection}, 
    slice::CubeSliceOrder
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Yellow,
    Blue,
    Red,
    Green,
    Orange,
    Magenta,
    Gray,
}

impl Color {
    pub fn to_ansi(&self) -> &str {
        match self {
            Color::White => "\x1b[97m",
            Color::Yellow => "\x1b[93m",
            Color::Blue  => "\x1b[94m",
            Color::Red   => "\x1b[91m",
            Color::Green => "\x1b[92m",
            Color::Orange => "\x1b[38;5;208m",
            Color::Magenta => "\x1b[95m",
            Color::Gray  => "\x1b[90m",
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct CubeMove {
    pub axis: Axis,
    pub grid_side: GridSide,
    pub order: CubeSliceOrder,
    pub direction: MoveDirection,
}

impl CubeMove {
    pub fn from_str(mv: &str) -> Result<CubeMove, String> {
        let (side_char, suffix) = mv.split_at(1);

        let (axis, grid_side, order) = match side_char {
            "R" => (Axis::X, GridSide::RIGHT, CubeSliceOrder::LAST),
            "L" => (Axis::X, GridSide::LEFT, CubeSliceOrder::FIRST),
            "U" => (Axis::Y, GridSide::TOP, CubeSliceOrder::FIRST),
            "D" => (Axis::Y, GridSide::BOTTOM, CubeSliceOrder::LAST),
            "F" => (Axis::Z, GridSide::FRONT, CubeSliceOrder::FIRST),
            "B" => (Axis::Z, GridSide::BACK, CubeSliceOrder::LAST),
            _ => return Err(format!("Incorrect move '{}'", mv)),
        };
        let direction = match suffix {
            "" => Ok(MoveDirection::Clockwise),
            "'" => Ok(MoveDirection::CounterClockwise),
            "2" => Ok(MoveDirection::Double),
            _ => Err(format!("Incorrect move '{}'", mv)),
        }?;

        Ok(CubeMove { axis, grid_side, order, direction })
    }
}