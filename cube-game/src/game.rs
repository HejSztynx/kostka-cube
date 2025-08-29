#![deny(clippy::all)]
#![forbid(unsafe_code)]

use cube_core::utils::cube_utils::Axis;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rusttype::Font;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use game_loop::{game_loop, Time, TimeTrait as _};

use cube_core::{
    cube::{
        core::{
            grid::{Grid, MoveDirection},
            scramble::scramble
        }, cube::Cube, slice::CubeMove, slice_builder::CubeSliceBuilder
    },
    game::render::Screen,
};

use core::f32;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use crate::{args::GameArgs, controls::{update_controls, Controls}, draw::draw, key_mapping::{SCRAMBLE_CODE, TIMER_CODE}, render::{animate_rotation, AnimatedMoveInfo}, timer::*};

const X_INIT: f32 = 0.0;
const Y_INIT: f32 = 0.0;
const Z_INIT: f32 = 5.0;
const ZP: f32 = 6.0;
const PROJECTION_SCALE: f32 = 64.0;

const FPS: u32 = 60;
const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

pub fn game(args: GameArgs) -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(args.width as f64, args.height as f64);
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
        Pixels::new(args.width, args.height, surface_texture)?
    };
    let game = Game::new(args, pixels);

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
            draw(&mut g.game);
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
                if g.game.input.key_pressed(SCRAMBLE_CODE) {
                    g.game.reset_game();
                }

                // Switch timer
                if g.game.input.key_pressed(TIMER_CODE) {
                    toggle_timer(&mut g.game);
                }

                // Update controls
                update_controls(&mut g.game);

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

pub struct Game {
    pub args: GameArgs,
    pub start: bool,
    pub screen: Screen,
    pub cube: Cube,
    pub grid: Grid,
    pub controls: Controls,
    pub input: WinitInputHelper,
    pub pixels: Pixels<'static>,
    pub font: Font<'static>,
    pub timer: Option<Timer>,
}

impl Game {
    pub fn new(args: GameArgs, pixels: Pixels<'static>) -> Game {
        let screen = Screen::new(ZP, PROJECTION_SCALE);

        let position: (f32, f32, f32) = (X_INIT, Y_INIT, Z_INIT);
        let angle_x = -f32::consts::FRAC_PI_4;
        let angle_y = f32::consts::FRAC_PI_4;

        let mut cube = Cube::new(position, angle_y, angle_x);
        let grid = Grid::new();
        cube.apply_grid(&grid);

        let controls = Controls::new();
        let input = WinitInputHelper::new();

        let font_data = include_bytes!("../assets/fonts/DotGothic16-Regular.ttf");
        let font = rusttype::Font::try_from_bytes(font_data as &[u8]).unwrap();

        Game {
            args,
            start: false,
            screen,
            cube,
            grid,
            controls, 
            input,
            pixels,
            font,
            timer: None,
        }
    }

    fn handle_next_move(&mut self, mv: CubeMove) {
        let mut translated_move = self.cube.translate_move(mv);
        if self.controls.double_move {
            translated_move.direction = MoveDirection::Double;
        }
        self.make_move(translated_move);
        self.controls.next_move = None;
    }

    fn handle_animation_step(&mut self, am_rc: Rc<RefCell<AnimatedMoveInfo>>) {
        let mut am = am_rc.borrow_mut();
        for slice in am.slices.iter_mut() {
            if self.controls.rotation_y != 0.0 {
                slice.rotate(Axis::Y, self.controls.rotation_y * self.args.rotation_angle);
            }
            if self.controls.rotation_x != 0.0 {
                slice.rotate(Axis::X, self.controls.rotation_x * self.args.rotation_angle);
            }
            if self.controls.rotation_z != 0.0 {
                slice.rotate(Axis::Z, self.controls.rotation_z * self.args.rotation_angle);
            }
        }

        if animate_rotation(self, &mut am) {
            self.controls.animated_move = Some(Rc::clone(&am_rc));
        }
    }

    fn update_cube_rotation(&mut self) {
        if self.controls.rotation_y != 0.0 {
            self.cube.rotate_y(self.controls.rotation_y * self.args.rotation_angle);
        }
        if self.controls.rotation_x != 0.0 {
            self.cube.rotate_x(self.controls.rotation_x * self.args.rotation_angle);
        }
        if self.controls.rotation_z != 0.0 {
            self.cube.rotate_z(self.controls.rotation_z * self.args.rotation_angle);
        }
    }

    fn update(&mut self) {
        if self.grid.is_solved() {
            stop_timer(self);
        }

        if let Some(am_rc) = self.controls.animated_move.take() {
            self.handle_animation_step(am_rc);
        } else {
            if let Some(mv) = self.controls.next_move.clone() {
                self.handle_next_move(mv);
            }
        }
        
        self.update_cube_rotation();

        self.cube.update_side_map();
        if self.controls.animated_move.is_none() {
            self.screen.clear_screen();
            self.screen.render(vec![&self.cube]);
        }
    }

    fn reset_game(&mut self) {
        self.start = false;
        reset_timer(self);
        scramble(&mut self.grid);
        self.cube.apply_grid(&self.grid);
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
}