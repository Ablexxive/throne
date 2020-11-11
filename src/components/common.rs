use bevy::prelude::*;

#[derive(Default)]
pub struct Paused(pub bool);

#[derive(Properties, Clone, Default)]
pub struct SpritePlaceholderMaterial(pub Handle<ColorMaterial>); // For entities without sprites.

// Used to specify entities that need their rotation locked.
#[derive(Properties, Default)]
pub struct LockRotation;
