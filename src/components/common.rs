use bevy::prelude::*;

#[derive(Default)]
pub struct Paused(pub bool);

#[derive(Reflect, Clone, Default)]
pub struct SpritePlaceholderMaterial(pub Handle<ColorMaterial>); // For entities without sprites.

// Used to specify entities that need their rotation locked.
#[derive(Reflect, Default)]
pub struct LockRotation;
