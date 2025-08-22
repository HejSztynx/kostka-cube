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
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 64.0;

const FRAME_TIME: u64 = 50;
const NO_STEPS: u8 = 8;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;

const FPS: u32 = 20;
const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

pub fn game() -> Result<(), Error> {
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
                // Close events
                if g.game.input.key_pressed(KeyCode::Escape) || g.game.input.close_requested() {
                    g.exit();
                    return;
                }

                // Reset game
                if g.game.input.key_pressed(KeyCode::ShiftLeft) {
                    g.game.reset_game();
                }

                // Update controls
                g.game.update_controls();

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

#[derive(Clone)]
struct AnimatedMoveInfo {
    slices: [CubeSlice; 3],
    slice_id: usize,
    current_step: u8,
    cube_move: CubeMove,
}

struct Controls {
    animated_move: Option<Rc<RefCell<AnimatedMoveInfo>>>,
    next_move: Option<CubeMove>,
    rotation_x: f32,
    rotation_y: f32,
}

impl Controls {
    fn new() -> Controls {
        Controls {
            animated_move: None,
            next_move: None,
            rotation_x: 0.0,
            rotation_y: 0.0,
        }
    }
}

struct Game {
    screen: Screen,
    cube: Cube,
    grid: Grid,
    controls: Controls,
    input: WinitInputHelper,
    pixels: Pixels<'static>,
}

impl Game {
    pub fn new(pixels: Pixels<'static>) -> Game {
        let screen = Screen::new(ZP, PROJECTION_SCALE);

        let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
        let angle_x = -f32::consts::FRAC_PI_4;
        let angle_y = f32::consts::FRAC_PI_4;

        let mut cube = Cube::new(position, angle_y, angle_x);
        let grid = Grid::new();
        cube.apply_grid(&grid);

        let controls = Controls::new();
        let input = WinitInputHelper::new();

        Game {
            screen,
            cube,
            grid,
            controls, 
            input,
            pixels,
        }
    }
    
    fn update_controls(&mut self) {
        use crate::controls::*;

        if self.input.key_held(ROTATE_X_CODE) {
            self.controls.rotation_x = 1.0;
        } else if self.input.key_held(ROTATE_X_PRIM_CODE) {
            self.controls.rotation_x = -1.0;
        } else {
            self.controls.rotation_x = 0.0;
        }
        if self.input.key_held(ROTATE_Y_CODE) {
            self.controls.rotation_y = 1.0;
        } else if self.input.key_held(ROTATE_Y_PRIM_CODE) {
            self.controls.rotation_y = -1.0;
        } else {
            self.controls.rotation_y = 0.0;
        }

        let next_move = move_bindings()
            .into_iter()
            .find(|(key_code, _)| self.input.key_pressed(*key_code))
            .map(|(_, (side, direction))| {
               CubeMove::from_side(side, direction)
            });


        self.controls.next_move = next_move;
    }
    
    fn update(&mut self) {
        let angle_unit = f32::consts::FRAC_PI_8 / 8.0;

        if let Some(am_rc) = self.controls.animated_move.take() {
            {
                let mut am = am_rc.borrow_mut();
                for slice in am.slices.iter_mut() {
                    if self.controls.rotation_y != 0.0 {
                        slice.rotate_y(self.controls.rotation_y * angle_unit);
                    }
                    if self.controls.rotation_x != 0.0 {
                        slice.rotate_x(self.controls.rotation_x * angle_unit);
                    }
                }

                if self.animate_rotation(&mut am) {
                    self.controls.animated_move = Some(Rc::clone(&am_rc));
                }
            }
        } else {
            if let Some(mv) = self.controls.next_move.clone() {
                let translated_move = self.cube.translate_move(mv);
                self.make_move(translated_move);
                self.controls.next_move = None;
            }
        }
        
        if self.controls.rotation_y != 0.0 {
            self.cube.rotate_y(self.controls.rotation_y * angle_unit);
        }
        if self.controls.rotation_x != 0.0 {
            self.cube.rotate_x(self.controls.rotation_x * angle_unit);
        }

        self.cube.update_side_map();
        if self.controls.animated_move.is_none() {
            self.screen.clear_screen();
            self.screen.render(vec![&self.cube]);
        }
    }

    fn draw(&mut self) {
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let rgba = if let Some(color) = self.screen.color_at(x, y) {
                color.rgba()
            } else {
                Color::Black.rgba()
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    fn reset_game(&mut self) {
        scramble(&mut self.grid);
        self.cube.apply_grid(&self.grid);
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

    fn make_move(&mut self, cube_move: CubeMove) {
        let slices = CubeSliceBuilder::create_cube_slices(&self.cube, &self.grid, &cube_move.axis);
        self.controls.animated_move = Some(Rc::new(RefCell::new(AnimatedMoveInfo {
            slices,
            slice_id: cube_move.order.idx(),
            current_step: 0,
            cube_move: cube_move.clone(),
        })));
        self.grid.apply_move(cube_move);
        self.cube.apply_grid(&self.grid);
    }

    fn render_once(&mut self, slices: &[CubeSlice; 3]) {
        self.screen.clear_screen();
        let slices_vec: Vec<&dyn Renderable> = slices.iter()
            .map(|s| s as &dyn Renderable)
            .collect();

        self.screen.render(slices_vec);
    }

    fn finish_animating_rotation(&mut self) {
        self.controls.animated_move = None;
        self.cube.apply_grid(&self.grid);
    }

    fn animate_rotation(&mut self, am: &mut AnimatedMoveInfo) -> bool {
        let angle_diff = Game::get_angle_diff(&am.cube_move);
        
        self.render_once(&am.slices);
        let slice_to_move = &mut am.slices[am.slice_id];
        slice_to_move.rotate_around_own_axis(angle_diff);
        am.current_step += 1;

        if am.current_step == NO_STEPS {
            self.finish_animating_rotation();
            false
        } else {
            true
        }
    }
}