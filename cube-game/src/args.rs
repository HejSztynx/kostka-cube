use std::f32::consts::PI;

pub struct GameArgs {
    pub width: u32,
    pub height: u32,
    pub rotation_angle: f32,
    pub no_steps: u8,
}

impl GameArgs {
    pub fn new(
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
}

pub enum Resolution {
    LOW,
    MEDIUM,
    HIGH,
}

impl Resolution {
    fn get_dimension(self) -> u32 {
        use self::Resolution::*;

        match self {
            LOW => 180,
            MEDIUM => 320,
            HIGH => 480,
        }
    }
}

pub enum RotationSpeed {
    SLOW,
    MEDIUM,
    FAST,
}

impl RotationSpeed {
    fn get_rotation_angle(self) -> f32 {
        use self::RotationSpeed::*;

        match self {
            SLOW => PI / 128.0,
            MEDIUM => PI / 64.0,
            FAST => PI / 32.0,
        }
    }
}

pub enum MoveSpeed {
    SLOW,
    MEDIUM,
    FAST,
}

impl MoveSpeed {
    fn get_no_steps(self) -> u8 {
        use self::MoveSpeed::*;

        match self {
            SLOW => 32,
            MEDIUM => 16,
            FAST => 8,
        }
    }
}