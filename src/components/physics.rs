use bevy::prelude::Vec2;

#[derive(Clone, Debug)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<Vec2> for Velocity {
    fn from(val: Vec2) -> Self {
        Self(val)
    }
}
