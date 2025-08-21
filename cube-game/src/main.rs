use cube_game::game;
use pixels::Error;

fn main() -> Result<(), Error> {
    game::game()?;

    Ok(())
}