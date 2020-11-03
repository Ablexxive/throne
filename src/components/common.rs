use bevy::prelude::*;

#[derive(Default)]
pub struct Paused(pub bool);

pub struct SpritePlaceholderMaterial(pub Handle<ColorMaterial>); // For entities without sprites.
