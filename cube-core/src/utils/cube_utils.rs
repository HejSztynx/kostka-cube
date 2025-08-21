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
            Color::Magenta => "\x1b[95m",
            Color::Gray  => "\x1b[90m",
            Color::Black => panic!()
        }
    }

    pub fn rgba(&self) -> [u8; 4] {
        match self {
            Color::White   => [0xff, 0xff, 0xff, 0xff],
            Color::Yellow  => [0xff, 0xff, 0x00, 0xff],
            Color::Blue    => [0x00, 0x00, 0xff, 0xff],
            Color::Red     => [0xff, 0x00, 0x00, 0xff],
            Color::Green   => [0x00, 0xff, 0x00, 0xff],
            Color::Orange  => [0xff, 0xa5, 0x00, 0xff],
            Color::Magenta => [0xff, 0x00, 0xff, 0xff],
            Color::Gray    => [0x80, 0x80, 0x80, 0xff],
            Color::Black   => [0x00, 0x00, 0x00, 0xff],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}