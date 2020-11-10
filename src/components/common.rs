use bevy::prelude::*;

#[derive(Default)]
pub struct Paused(pub bool);

pub struct SpritePlaceholderMaterial(pub Handle<ColorMaterial>); // For entities without sprites.

// Used to specify entities that need their rotation locked.
pub struct LockRotation;
