use crate::{
    utils::{
        cube_utils::Axis,
    },
    cube::{
        core::grid::{Grid, GridSide, MoveDirection}
    }
};

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Move {
    mv: &'static str,
    axis: Axis,
}

const MOVES: &[Move] = &[
    Move { mv: "R", axis: Axis::X },
    Move { mv: "R'", axis: Axis::X },
    Move { mv: "R2", axis: Axis::X },
    Move { mv: "L", axis: Axis::X },
    Move { mv: "L'", axis: Axis::X },
    Move { mv: "L2", axis: Axis::X },

    Move { mv: "U", axis: Axis::Y },
    Move { mv: "U'", axis: Axis::Y },
    Move { mv: "U2", axis: Axis::Y },
    Move { mv: "D", axis: Axis::Y },
    Move { mv: "D'", axis: Axis::Y },
    Move { mv: "D2", axis: Axis::Y },

    Move { mv: "F", axis: Axis::Z },
    Move { mv: "F'", axis: Axis::Z },
    Move { mv: "F2", axis: Axis::Z },
    Move { mv: "B", axis: Axis::Z },
    Move { mv: "B'", axis: Axis::Z },
    Move { mv: "B2", axis: Axis::Z },
];

fn generate_scramble(length: usize) -> Vec<&'static Move> {
    let mut rng = thread_rng();
    let mut scramble = Vec::with_capacity(length);

    let mut last_move: Option<&Move> = None;
    for _ in 0..length {
        let choices: Vec<&Move> = MOVES.iter()
            .filter(|m| {
                match last_move {
                    Some(prev) => prev.axis != m.axis,
                    None => true,
                }
            })
            .collect();

        let selected = *choices.choose(&mut rng).unwrap();
        scramble.push(selected);
        last_move = Some(selected);
    }

    scramble
}

fn _print_scramble(moves: &Vec<&Move>) {
    println!("Scramble:");
    for &mv in moves {
        print!("{} ", mv.mv);
    }
    println!();
}

fn apply_scramble(grid: &mut Grid, moves: Vec<&Move>) {
    for mv in moves {
        let (side_char, suffix) = mv.mv.split_at(1);
        let side = match side_char {
            "R" => GridSide::Right,
            "L" => GridSide::Left,
            "U" => GridSide::Top,
            "D" => GridSide::Bottom,
            "F" => GridSide::Front,
            "B" => GridSide::Back,
            _ => continue,
        };
        let direction = match suffix {
            "" => MoveDirection::Clockwise,
            "'" => MoveDirection::CounterClockwise,
            "2" => {
                grid.move_face(side, MoveDirection::Clockwise);
                grid.move_face(side, MoveDirection::Clockwise);
                continue;
            },
            _ => continue,
        };

        grid.move_face(side, direction);
    }
}

pub fn scramble(grid: &mut Grid) {
    let scramble = generate_scramble(20);
    apply_scramble(grid, scramble);
}