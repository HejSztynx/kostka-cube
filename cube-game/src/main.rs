#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use game_loop::{game_loop, Time, TimeTrait as _};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;
const BOX_SIZE: i16 = 64;

const FPS: u32 = 20;
const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = WindowBuilder::new()
            .with_title("Kostka")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();
        Arc::new(window)
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let game = Game::new(pixels);

    let res = game_loop(
        event_loop,
        window,
        game,
        FPS as u32,
        0.1,
        move |g| {
            g.game.update();
        },
        move |g| {
            // Drawing
            g.game.draw();
            if let Err(err) = g.game.pixels.render() {
                log_error("pixels.render", err);
                g.exit();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174
            let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if dt > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(dt));
            }
        },
        |g, event| {
            // Let winit_input_helper collect events to build its state.
            if g.game.input.update(event) {
                // Update controls
                // g.game.update_controls();

                // Close events
                if g.game.input.key_pressed(KeyCode::Escape) || g.game.input.close_requested() {
                    g.exit();
                    return;
                }

                // Reset game
                // if g.game.input.key_pressed(KeyCode::KeyR) {
                //     g.game.reset_game();
                // }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        g.exit();
                    }
                }
            }
        },
    );
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

use cube_core::{
    cube::{
        core::{
            grid::{Grid, GridSide, MoveDirection},
            scramble::scramble
        }, cube::Cube, slice::{CubeMove, CubeSlice, CubeSliceOrder}, slice_builder::CubeSliceBuilder
    },
    game::render::{Renderable, Screen},
    utils::cube_utils::Color
};

use core::f32;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 64.0;

const FRAME_TIME: u64 = 50;
const NO_STEPS: u8 = 8;

struct Game {
    screen: Screen,
    cube: Cube,
    grid: Grid,
    input: WinitInputHelper,
    pixels: Pixels<'static>,
}

impl Game {
    pub fn new(pixels: Pixels<'static>) -> Game {
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
            input: WinitInputHelper::new(),
            pixels,
        }
    }
    
    fn update(&mut self) {
        let angle_unit = f32::consts::FRAC_PI_8 / 8.0;

        self.cube.update_side_map();
        self.cube.apply_grid(&self.grid);
        self.screen.clear_screen();
        self.screen.render(vec![&self.cube]);

        self.cube.rotate_y(angle_unit);
        self.cube.rotate_x(angle_unit);
    }

    fn draw(&mut self) {
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let rgba = if let Some(_color) = self.screen.color_at(x, y) {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
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

        let mut slices = CubeSliceBuilder::create_cube_slices(&self.cube, &self.grid, &cube_move.axis);
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
        self.screen.clear_screen();
        let slices_vec: Vec<&dyn Renderable> = slices.iter()
        .map(|s| s as &dyn Renderable)
        .collect();

        self.screen.render(slices_vec);
        self.screen.print_screen();
        std::thread::sleep(Duration::from_millis(FRAME_TIME));
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