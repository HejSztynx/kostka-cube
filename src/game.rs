use crate::{cube::Cube, grid::MoveDirection};
use crate::grid::{Grid, GridSide};
use crate::screen::{Renderable, Screen};
use crate::slice::{CubeMove, CubeSlice, CubeSliceOrder};
use crate::scramble::scramble;

use core::f32;
use std::io::{self, Write};
use std::time::Duration;

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 10.0;

const FRAME_TIME: u64 = 50;
const NO_STEPS: u8 = 8;

struct Game {
    screen: Screen,
    cube: Cube,
    grid: Grid,
}

impl Game {
    pub fn new() -> Game {
        let screen = Screen::new(ZP, PROJECTION_SCALE);

        let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
        let angle_x = -f32::consts::FRAC_PI_4;
        let angle_y = f32::consts::FRAC_PI_4;

        let cube = Cube::new(position, angle_y, angle_x);

        let grid = Grid::new();

        Game {
            screen,
            cube,
            grid,
        }
    }
    
    fn start(&mut self) {        
        let angle_unit = f32::consts::FRAC_PI_8;
        
        self.screen.clear_screen(); 
        loop {
            self.cube.update_side_map();
            self.cube.apply_grid(&self.grid);
            self.screen.render(vec![&self.cube]);
            self.screen.print_screen();
            
            let input = self.get_user_input();
            
            match input.as_str() {
                "" => {},
                "quit" => break,
                "scramble" => scramble(&mut self.grid),
                "a" => self.cube.rotate_y(angle_unit),
                "d" => self.cube.rotate_y(-angle_unit),
                "w" => self.cube.rotate_x(angle_unit),
                "s" => self.cube.rotate_x(-angle_unit),
                _ => {
                    if let Err(err) = self.make_move(input.as_str()) {
                        println!("{}", err);
                        let _ = io::stdin().read_line(&mut String::new());
                        self.screen.clear_screen(); 
                    }
                }
            }
        }
    }

    fn get_user_input(&mut self) -> String {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.screen.clear_screen(); 
        
        input.trim().to_string()
    }

    fn get_angle_diff(cube_move: &CubeMove) -> f32 {
        let mut angle_diff = f32::consts::FRAC_PI_2 / (NO_STEPS as f32);
        
        if let MoveDirection::CounterClockwise = cube_move.direction {
            angle_diff *= -1.0;
        }
        
        if let CubeSliceOrder::LAST = cube_move.order {
            angle_diff *= -1.0;
        }
        
        if let MoveDirection::Double = cube_move.direction {
            angle_diff *= 2.0;
        }

        if let GridSide::MiddleY = cube_move.grid_side {
            angle_diff *= -1.0;
        }
        
        angle_diff
    }

    fn make_move(&mut self, input: &str) -> Result<(), String> {
        let cube_move = self.parse_and_translate_move(input)?;
        
        print!("Translated move - {:?}", cube_move);
        io::stdout().flush().unwrap();

        std::thread::sleep(Duration::from_millis(2000));

        let mut slices = self.cube.create_cube_slices(&self.grid, &cube_move.axis);
        self.animate_rotation(&mut slices, &cube_move);
        
        self.grid.apply_move(cube_move);
        Ok(())
    }

    fn parse_and_translate_move(&self, input: &str) -> Result<CubeMove, String> {
        let (side, direction) = CubeMove::from_str(input)?;
        let cube_move = CubeMove::from_side(side, direction);
        let translated_move = self.cube.translate_move(cube_move);
        Ok(translated_move)
    }

    fn render_once(&mut self, slices: &[CubeSlice; 3]) {
        let slices_vec: Vec<&dyn Renderable> = slices.iter()
        .map(|s| s as &dyn Renderable)
        .collect();

        self.screen.render(slices_vec);
        self.screen.print_screen();
        std::thread::sleep(Duration::from_millis(FRAME_TIME));
        self.screen.clear_screen();
    }

    fn animate_rotation(&mut self, slices: &mut [CubeSlice; 3], cube_move: &CubeMove) {
        let angle_diff = Game::get_angle_diff(cube_move);
        
        self.render_once(slices);
        
        for _ in 0..NO_STEPS {
            let slice_to_move = &mut slices[cube_move.order.idx()];
            slice_to_move.rotate_around_own_axis(angle_diff);
            self.render_once(slices);
        }
    }
}

pub fn game() {
    let mut game = Game::new();
    game.start();
}