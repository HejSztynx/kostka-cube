use std::f32::consts::FRAC_PI_2;

use cube_core::cube::core::grid::GridSide;
use cube_core::cube::slice::CubeSliceOrder;
use cube_core::{cube::{core::grid::MoveDirection, slice::{CubeMove, CubeSlice}}, game::render::Renderable};
use crate::game::Game;

pub struct AnimatedMoveInfo {
    pub slices: [CubeSlice; 3],
    pub slice_id: usize,
    pub current_step: u8,
    pub cube_move: CubeMove,
}

fn render_animation_frame(game: &mut Game, slices: &[CubeSlice; 3]) {
    game.screen.clear_screen();
    let slices_vec: Vec<&dyn Renderable> = slices.iter()
        .map(|s| s as &dyn Renderable)
        .collect();

    game.screen.render(slices_vec);
}

fn finish_animating_rotation(game: &mut Game) {
    game.controls.animated_move = None;
    game.cube.apply_grid(&game.grid);
}

pub fn animate_rotation(game: &mut Game, am: &mut AnimatedMoveInfo) -> bool {
    let angle_diff = get_angle_diff(game.args.no_steps, &am.cube_move);
    
    render_animation_frame(game, &am.slices);
    let slice_to_move = &mut am.slices[am.slice_id];
    slice_to_move.rotate_around_own_axis(angle_diff);
    am.current_step += 1;

    if am.current_step == game.args.no_steps {
        finish_animating_rotation(game);
        false
    } else {
        true
    }
}

fn get_angle_diff(no_steps: u8, cube_move: &CubeMove) -> f32 {
    let mut angle_diff = FRAC_PI_2 / (no_steps as f32);
    
    if let MoveDirection::CounterClockwise = cube_move.direction {
        angle_diff *= -1.0;
    }
    
    if let CubeSliceOrder::LAST = cube_move.order {
        angle_diff *= -1.0;
    }
    
    if let MoveDirection::Double = cube_move.direction {
        angle_diff *= 2.0;
    }

    if let GridSide::MiddleY = cube_move.grid_side {
        angle_diff *= -1.0;
    }
    
    angle_diff
}