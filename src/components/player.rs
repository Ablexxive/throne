use bevy::prelude::Reflect;

#[derive(Reflect)]
pub struct Player {
    pub move_speed: f32,
}

impl Player {
    pub fn new(move_speed: f32) -> Self {
        Self { move_speed }
    }
}
