use std::{thread, time::Duration};

use kostka::cube::Cube;
use kostka::game::game;
use kostka::grid::Grid;
use kostka::screen::Screen;

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 10.0;

fn cube() {
    let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    let grid = Grid::new();

    Screen::clear_screen();
    loop {
        let cube = Cube::new(position, angle_y, angle_x);

        let mut screen = Screen::new(ZP, PROJECTION_SCALE);
        
        // cube.apply_grid(&grid);
        screen.render_cube(&cube);
        screen.print_screen();
        thread::sleep(Duration::from_millis(100));
        Screen::clear_screen();

        angle_y += 0.1;
        angle_x += 0.1;
    }
}

fn main() {
    cube();

    // game();
}