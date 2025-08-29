use std::{cell::RefCell, rc::Rc};

use cube_core::cube::slice::CubeMove;

use crate::{game::Game, render::AnimatedMoveInfo, timer::start_timer};

pub struct Controls {
    pub animated_move: Option<Rc<RefCell<AnimatedMoveInfo>>>,
    pub next_move: Option<CubeMove>,
    pub double_move: bool,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            animated_move: None,
            next_move: None,
            double_move: false,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }
}

fn update_controls_rotations(game: &mut Game) {
    use crate::key_mapping::*;

    if game.input.key_held(ROTATE_X_CODE) {
        game.controls.rotation_x = 1.0;
    } else if game.input.key_held(ROTATE_X_PRIM_CODE) {
        game.controls.rotation_x = -1.0;
    } else {
        game.controls.rotation_x = 0.0;
    }
    if game.input.key_held(ROTATE_Y_CODE) {
        game.controls.rotation_y = 1.0;
    } else if game.input.key_held(ROTATE_Y_PRIM_CODE) {
        game.controls.rotation_y = -1.0;
    } else {
        game.controls.rotation_y = 0.0;
    }
    if game.input.key_held(ROTATE_Z_CODE) {
        game.controls.rotation_z = 1.0;
    } else if game.input.key_held(ROTATE_Z_PRIM_CODE) {
        game.controls.rotation_z = -1.0;
    } else {
        game.controls.rotation_z = 0.0;
    }
}

pub fn update_controls(game: &mut Game) {
    use crate::key_mapping::*;

    update_controls_rotations(game);

    game.controls.double_move = game.input.key_held(DOUBLE_MOVE);

    let next_move = move_bindings()
        .into_iter()
        .find(|(key_code, _)| game.input.key_pressed(*key_code))
        .map(|(_, (side, direction))| {
            CubeMove::from_side(side, direction)
        }
    );

    if next_move.is_some() {
        game.controls.next_move = next_move;
        if !game.start {
            game.start = true;
            start_timer(game);
        }
    }
}