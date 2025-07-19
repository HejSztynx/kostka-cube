use crate::{cube::{Cube, Axis}, grid::Grid, scramble::scramble, screen::Screen};
use core::f32;
use std::io::{self, Write};

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 10.0;

fn start(grid: &mut Grid) {
    let mut screen = Screen::new(ZP, PROJECTION_SCALE);

    let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    let angle_unit = f32::consts::FRAC_PI_8;
    
    screen.clear_screen(); 
    loop {
        let mut cube = Cube::new(position, angle_y, angle_x);
        
        cube.apply_grid(grid);
        screen.render(&cube);
        screen.print_screen();
        
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        screen.clear_screen(); 
        
        let input = input.trim();
        
        match input {
            "" => {},
            "quit" => break,
            "scramble" => scramble(grid),
            "a" => angle_y += angle_unit,
            "d" => angle_y -= angle_unit,
            "w" => angle_x += angle_unit,
            "s" => angle_x -= angle_unit,
            _ => match grid.apply_move(input) {
                Ok(_) => {
                    cube.apply_grid(grid);
                    let slices = cube.create_cube_slices(grid, Axis::X);
                    screen.render(&slices[0]);
                    screen.print_screen();
                    let _ = io::stdin().read_line(&mut String::new());
                    screen.clear_screen();
                    
                    screen.render(&slices[1]);
                    screen.print_screen();
                    let _ = io::stdin().read_line(&mut String::new());
                    screen.clear_screen(); 
                    
                    screen.render(&slices[2]);
                    screen.print_screen();
                    let _ = io::stdin().read_line(&mut String::new());
                    screen.clear_screen(); 
                }
                Err(err) => {
                    println!("{}", err);
                    let _ = io::stdin().read_line(&mut String::new());
                    screen.clear_screen(); 
                }
            }
        }
    }
}

pub fn game() {
    let mut grid = Grid::new();
    start(&mut grid);
}