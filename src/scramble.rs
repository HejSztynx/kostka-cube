use crate::grid::{Grid, GridSide, MoveDirection};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    notation: &'static str,
    axis: Axis,
}

const MOVES: &[&str] = &[
    "R",
    "R'",
    "R2",
    "L",
    "L'",
    "L2",

    "U",
    "U'",
    "U2",
    "D",
    "D'",
    "D2",

    "F",
    "F'",
    "F2",
    "B",
    "B'",
    "B2",
];

fn generate_scramble(length: usize) -> Vec<&'static str> {
    let mut rng = thread_rng();
    let mut scramble = Vec::with_capacity(length);

    let mut last_move: Option<&str> = None;
    for _ in 0..length {
        let choices: Vec<&str> = MOVES.iter()
            .copied()
            .filter(|m| {
                // avoid same axis immediately
                match last_move {
                    Some(prev) => prev[0..1] != m[0..1],
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

fn print_scramble(moves: &Vec<&str>) {
    println!("Scramble:");
    for &mv in moves {
        print!("{} ", mv);
    }
    println!();
}

fn apply_scramble(grid: &mut Grid, moves: Vec<&str>) {
    for mv in moves {
        let (side_char, suffix) = mv.split_at(1);
        let side = match side_char {
            "R" => GridSide::RIGHT,
            "L" => GridSide::LEFT,
            "U" => GridSide::TOP,
            "D" => GridSide::BOTTOM,
            "F" => GridSide::FRONT,
            "B" => GridSide::BACK,
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
    // print_scramble(&scramble);
    apply_scramble(grid, scramble);
}