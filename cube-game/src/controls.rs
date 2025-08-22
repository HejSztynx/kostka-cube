use std::collections::HashMap;

use cube_core::cube::core::grid::{GridSide, MoveDirection};
use winit::keyboard::KeyCode;

pub const ROTATE_X_CODE: KeyCode = KeyCode::KeyS;
pub const ROTATE_X_PRIM_CODE: KeyCode = KeyCode::KeyW;
pub const ROTATE_Y_CODE: KeyCode = KeyCode::KeyA;
pub const ROTATE_Y_PRIM_CODE: KeyCode = KeyCode::KeyD;

pub const MOVE_R_CODE: KeyCode = KeyCode::KeyR;
pub const MOVE_R_PRIM_CODE: KeyCode = KeyCode::KeyT;

pub fn move_bindings() -> HashMap<KeyCode, (GridSide, MoveDirection)> {
    use GridSide::*;
    use MoveDirection::*;

    HashMap::from([
        (MOVE_R_CODE, (Right, Clockwise)),
        (MOVE_R_PRIM_CODE, (Right, CounterClockwise)),

        // (KeyCode::KeyL, (Left, Clockwise)),
        // (KeyCode::KeyK, (Left, CounterClockwise)),

        // (KeyCode::KeyU, (Top, Clockwise)),
        // (KeyCode::KeyJ, (Top, CounterClockwise)),

        // (KeyCode::KeyD, (Bottom, Clockwise)),
        // (KeyCode::KeyF, (Bottom, CounterClockwise)),

        // (KeyCode::KeyF, (Front, Clockwise)),
        // (KeyCode::KeyG, (Front, CounterClockwise)),

        // (KeyCode::KeyB, (Back, Clockwise)),
        // (KeyCode::KeyN, (Back, CounterClockwise)),
    ])
}