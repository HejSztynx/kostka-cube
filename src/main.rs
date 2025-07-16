use std::{thread, time::Duration};

use kostka::cube::Cubie;
use kostka::screen::Screen;
use kostka::grid::*;

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 5.0;
const PROJECTION_SCALE: f32 = 10.0;

fn cube() {
    let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    Screen::clear_screen();
    loop {
        let cubie = Cubie::new(position, angle_y, angle_x);
        
        let mut screen = Screen::new(ZP, PROJECTION_SCALE);
        
        screen.render_cubie(&cubie);
        screen.print_screen();
        thread::sleep(Duration::from_millis(100));
        Screen::clear_screen();

        angle_y += 0.1;
        angle_x += 0.1;
    }
}

fn main() {
    // cube();

    grid();
}

fn grid() {
    let mut grid = Grid::new();

    grid.print();
    // grid.move_face(GridSide::FRONT, MoveDirection::Clockwise);
    // grid.print();
    grid.move_face(GridSide::RIGHT, MoveDirection::Clockwise);
    grid.print();
    grid.move_face(GridSide::RIGHT, MoveDirection::CounterClockwise);
    grid.print();
    // grid.move_face(GridSide::FRONT, MoveDirection::Clockwise);
    // grid.print();
    

    // let mut face = Face::new();
    // face.print();
    
    // for _ in 0..5 {
        // println!("rotating clockwise...");
        // face.rotate_clockwise();
        // face.print();
    // }
    // for _ in 0..5 {
        // println!("rotating counter-clockwise...");
        // face.rotate_counter_clockwise();
        // face.print();
    // }
}