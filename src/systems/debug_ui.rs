use crate::components::Player;

use bevy::prelude::*;

pub fn debug_ui_update(
    player_info: Query<(&Player, &Transform)>,
    mut text_query: Query<&mut Text>,
) {
    for mut text in text_query.iter_mut() {
        for (_player, transform) in player_info.iter() {
            text.value = format!(
                "Player Pos: {:.2}, {:.2}",
                transform.translation.x(),
                transform.translation.y()
            );
        }
    }
}
