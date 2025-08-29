use std::f32::consts::PI;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about = "Kostka Cube", long_about = None)]
struct Cli {
    /// Set resolution (low, medium, high)
    #[arg(long, value_enum, default_value_t = Resolution::Medium)]
    res: Resolution,

    /// Set rotation speed (low, medium, high)
    #[arg(long, value_enum, default_value_t = RotationSpeed::Medium)]
    rs: RotationSpeed,

    /// Set move speed (low, medium, high)
    #[arg(long, value_enum, default_value_t = MoveSpeed::Medium)]
    ms: MoveSpeed,
}

pub struct GameArgs {
    pub width: u32,
    pub height: u32,
    pub rotation_angle: f32,
    pub no_steps: u8,
}

impl GameArgs {
    fn new(
        resolution: Resolution,
        rotation_speed: RotationSpeed,
        move_speed: MoveSpeed,
    ) -> GameArgs {
        let dimension = resolution.get_dimension();

        GameArgs {
            width: dimension,
            height: dimension,
            rotation_angle: rotation_speed.get_rotation_angle(),
            no_steps: move_speed.get_no_steps(),
        }
    }

    pub fn parse() -> GameArgs {
        let cli = Cli::parse();

        GameArgs::new(
            cli.res,
            cli.rs,
            cli.ms,
        )
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Resolution {
    Low,
    Medium,
    High,
}

impl Resolution {
    fn get_dimension(self) -> u32 {
        use self::Resolution::*;

        match self {
            Low => 180,
            Medium => 320,
            High => 480,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RotationSpeed {
    Low,
    Medium,
    High,
}

impl RotationSpeed {
    fn get_rotation_angle(self) -> f32 {
        use self::RotationSpeed::*;

        match self {
            Low => PI / 128.0,
            Medium => PI / 64.0,
            High => PI / 32.0,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum MoveSpeed {
    Low,
    Medium,
    High,
}

impl MoveSpeed {
    fn get_no_steps(self) -> u8 {
        use self::MoveSpeed::*;

        match self {
            Low => 32,
            Medium => 16,
            High => 8,
        }
    }
}