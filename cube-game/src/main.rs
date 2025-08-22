use kostka::game;
use pixels::Error;

fn main() -> Result<(), Error> {
    game::game()?;

    Ok(())
}