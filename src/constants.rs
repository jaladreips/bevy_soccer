pub const FIELD_SIZE: bevy::prelude::Vec2 = bevy::prelude::Vec2::new(120.0, 80.0); // soccer field size (in meters)
pub const TICKRATE: i16 = 240; // ticks per second
pub const IMPULSE_PER_KEYPRESS: f32 = 100.0;
pub const PLAYER_RADIUS: f32 = 2.0;
pub const BALL_RADIUS: f32 = 0.9;
pub const EJECTION_IMPULSE: f32 = 1000.0; // this value states how much 2 bodies resist sinking when colliding
pub const PLAYER_MASS: f32 = 80.0;
pub const BALL_MASS: f32 = f32::INFINITY; // 10.0;
