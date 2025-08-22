#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Yellow,
    Blue,
    Red,
    Green,
    Orange,
    Gray,
    Black,
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
            Color::Gray  => "\x1b[90m",
            Color::Black => panic!()
        }
    }

    pub fn rgba(&self) -> [u8; 4] {
        match self {
            Color::White   => [235, 235, 235, 0xff],
            Color::Yellow  => [239, 249, 102, 0xff],
            Color::Blue    => [74, 150, 221, 0xff],
            Color::Red     => [249, 44, 59, 0xff],
            Color::Green   => [118, 242, 139, 0xff],
            Color::Orange  => [254, 146, 43, 0xff],
            Color::Gray    => [160, 152, 160, 0xff],
            Color::Black   => [35, 32, 47, 0xff],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}