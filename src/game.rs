use crate::{cube::Cube, grid::MoveDirection};
use crate::grid::Grid;
use crate::screen::{Renderable, Screen};
use crate::slice::{CubeMove, CubeSliceOrder};
use crate::scramble::scramble;

use core::f32;
use std::io::{self, Write};
use std::time::Duration;

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
        screen.render(vec![&cube]);
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
            _ => {
                if let Err(err) = make_move(input, grid, &mut screen, &mut cube) {
                    println!("{}", err);
                    let _ = io::stdin().read_line(&mut String::new());
                    screen.clear_screen(); 
                }
            }
        }
    }
}

fn get_angle_diff(no_steps: i32, cube_move: &CubeMove) -> f32 {
    let mut angle_diff = f32::consts::FRAC_PI_2 / (no_steps as f32);

    if let MoveDirection::CounterClockwise = cube_move.direction {
        angle_diff *= -1.0;
    }

    if let CubeSliceOrder::LAST = cube_move.order {
        angle_diff *= -1.0;
    }

    if let MoveDirection::Double = cube_move.direction {
        angle_diff *= 2.0;
    }

    angle_diff
}

fn make_move(input: &str, grid: &mut Grid, screen: &mut Screen, cube: &mut Cube) -> Result<(), String> {
    let (side, direction) = CubeMove::from_str(input)?;

    let translated_side = cube.translate_side(side);
    let cube_move = CubeMove::from_side(translated_side, direction);
    
    let mut slices = cube.create_cube_slices(grid, &cube_move.axis);
    
    let no_steps = 8;
    let angle_diff = get_angle_diff(no_steps, &cube_move);
    
    let slices_vec: Vec<&dyn Renderable> = slices.iter()
        .map(|s| s as &dyn Renderable)
        .collect();
    screen.render(slices_vec);
    screen.print_screen();
    std::thread::sleep(Duration::from_millis(50));
    screen.clear_screen();
    
    for _ in 0..no_steps {
        let slice_to_move = &mut slices[cube_move.order.idx()];
        slice_to_move.rotate_around_own_axis(angle_diff);
        
        let slices_vec: Vec<&dyn Renderable> = slices.iter()
            .map(|s| s as &dyn Renderable)
            .collect();

        screen.render(slices_vec);
        screen.print_screen();
        std::thread::sleep(Duration::from_millis(50));
        screen.clear_screen();
    }


    grid.apply_move(cube_move);

    Ok(())
}

pub fn game() {
    let mut grid = Grid::new();
    start(&mut grid);
}