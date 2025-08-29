mod game;
mod key_mapping;
mod render;
mod timer;
mod controls;
mod draw;
mod args;

use pixels::Error;

use crate::args::*;

fn main() -> Result<(), Error> {
    let resolution = Resolution::MEDIUM;
    let rotation_speed = RotationSpeed::MEDIUM;
    let move_speed = MoveSpeed::MEDIUM;

    let args = GameArgs::new(
        resolution,
        rotation_speed,
        move_speed
    );

    game::game(args)?;

    Ok(())
}