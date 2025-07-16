use crate::{grid::Grid, scramble::scramble, screen::Screen};
use std::io::{self, Write};

fn start(grid: &mut Grid) {
    loop {
        Screen::clear_screen();
        grid.print();
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        match input {
            "q" => break,
            "scramble" => scramble(grid),
            _ => match grid.apply_move(input) {
                Ok(_) => { }
                Err(err) => {
                    println!("{}", err);
                    let _ = io::stdin().read_line(&mut String::new());
                }
            }
        }
    }
}

pub fn game() {
    let mut grid = Grid::new();
    start(&mut grid);
}