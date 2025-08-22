use std::collections::HashMap;

use cube_core::cube::core::grid::{GridSide, MoveDirection};
use winit::keyboard::KeyCode;

// rotation key mapping

pub const ROTATE_X_CODE: KeyCode = KeyCode::KeyP;
pub const ROTATE_X_PRIM_CODE: KeyCode = KeyCode::Semicolon;

pub const ROTATE_Y_CODE: KeyCode = KeyCode::KeyC;
pub const ROTATE_Y_PRIM_CODE: KeyCode = KeyCode::KeyM;

pub const ROTATE_Z_CODE: KeyCode = KeyCode::KeyV;
pub const ROTATE_Z_PRIM_CODE: KeyCode = KeyCode::KeyN;

// move key mapping

pub const DOUBLE_MOVE: KeyCode = KeyCode::Space;

pub const MOVE_R_CODE: KeyCode = KeyCode::KeyI;
pub const MOVE_R_PRIM_CODE: KeyCode = KeyCode::KeyK;

pub const MOVE_L_CODE: KeyCode = KeyCode::KeyD;
pub const MOVE_L_PRIM_CODE: KeyCode = KeyCode::KeyE;

pub const MOVE_U_CODE: KeyCode = KeyCode::KeyU;
pub const MOVE_U_PRIM_CODE: KeyCode = KeyCode::KeyR;

pub const MOVE_D_CODE: KeyCode = KeyCode::KeyS;
pub const MOVE_D_PRIM_CODE: KeyCode = KeyCode::KeyL;

pub const MOVE_F_CODE: KeyCode = KeyCode::KeyJ;
pub const MOVE_F_PRIM_CODE: KeyCode = KeyCode::KeyF;

pub const MOVE_B_CODE: KeyCode = KeyCode::KeyW;
pub const MOVE_B_PRIM_CODE: KeyCode = KeyCode::KeyO;

pub const MOVE_M_CODE: KeyCode = KeyCode::KeyA;
pub const MOVE_M_PRIM_CODE: KeyCode = KeyCode::KeyQ;

pub const MOVE_E_CODE: KeyCode = KeyCode::KeyG;
pub const MOVE_E_PRIM_CODE: KeyCode = KeyCode::KeyH;

pub const MOVE_S_CODE: KeyCode = KeyCode::KeyY;
pub const MOVE_S_PRIM_CODE: KeyCode = KeyCode::KeyT;

pub fn move_bindings() -> HashMap<KeyCode, (GridSide, MoveDirection)> {
    use GridSide::*;
    use MoveDirection::*;

    HashMap::from([
        (MOVE_R_CODE, (Right, Clockwise)),
        (MOVE_R_PRIM_CODE, (Right, CounterClockwise)),

        (MOVE_L_CODE, (Left, Clockwise)),
        (MOVE_L_PRIM_CODE, (Left, CounterClockwise)),

        (MOVE_U_CODE, (Top, Clockwise)),
        (MOVE_U_PRIM_CODE, (Top, CounterClockwise)),

        (MOVE_D_CODE, (Bottom, Clockwise)),
        (MOVE_D_PRIM_CODE, (Bottom, CounterClockwise)),

        (MOVE_F_CODE, (Front, Clockwise)),
        (MOVE_F_PRIM_CODE, (Front, CounterClockwise)),

        (MOVE_B_CODE, (Back, Clockwise)),
        (MOVE_B_PRIM_CODE, (Back, CounterClockwise)),

        (MOVE_M_CODE, (MiddleX, Clockwise)),
        (MOVE_M_PRIM_CODE, (MiddleX, CounterClockwise)),

        (MOVE_E_CODE, (MiddleY, Clockwise)),
        (MOVE_E_PRIM_CODE, (MiddleY, CounterClockwise)),
        
        (MOVE_S_CODE, (MiddleZ, Clockwise)),
        (MOVE_S_PRIM_CODE, (MiddleZ, CounterClockwise)),
    ])
}