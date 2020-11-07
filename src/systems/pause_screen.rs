use bevy::prelude::*;

use crate::components::Paused;

pub struct PauseScreenItem;

pub fn pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut paused: ResMut<Paused>,
    mut query: Query<(&mut Draw, &PauseScreenItem)>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        paused.0 = !paused.0;
        if paused.0 {
            // Show elements on pause screen.
            for (mut draw_component, _filter) in query.iter_mut() {
                draw_component.is_visible = true;
            }
        } else {
            // Hide elements on pause screen.
            for (mut draw_component, _filter) in query.iter_mut() {
                draw_component.is_visible = false;
            }
        }
    }
}
