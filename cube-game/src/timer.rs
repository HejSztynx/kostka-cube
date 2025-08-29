use std::time::Instant;

use crate::game::Game;

pub struct Timer {
    elapsed: f32,
    run: bool,
    instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            elapsed: 0.0,
            run: false,
            instant: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.run = true;
    }

    pub fn stop(&mut self) {
        self.run = false;
    }

    pub fn update_elapsed(&mut self) -> f32 {
        if self.run {
            self.elapsed = self.instant.elapsed().as_secs_f32();
        }
        self.elapsed
    }
}

pub fn toggle_timer(game: &mut Game) {
    match &mut game.timer {
        Some(_) => game.timer = None,
        None if !game.start => game.timer = Some(Timer::new()),
        None => {}
    }
}

pub fn start_timer(game: &mut Game) {
    if let Some(timer) = &mut game.timer {
        timer.start();
    }
}

pub fn stop_timer(game: &mut Game) {
    if let Some(timer) = &mut game.timer {
        timer.stop();
    }
}

pub fn reset_timer(game: &mut Game) {
    match &mut game.timer {
        Some(_) => game.timer = Some(Timer::new()),
        None => {}
    }
}