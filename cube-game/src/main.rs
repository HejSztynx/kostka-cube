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
    let args = GameArgs::parse();

    game::game(args)?;

    Ok(())
}