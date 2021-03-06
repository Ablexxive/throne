use bevy::input::gamepad::{Gamepad, GamepadEvent, GamepadEventType};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct GamepadLobby {
    pub gamepads: HashSet<Gamepad>,
    pub gamepad_event_reader: EventReader<GamepadEvent>,
}

pub fn connection_system(
    mut lobby: ResMut<GamepadLobby>,
    gamepad_event: Res<Events<GamepadEvent>>,
) {
    for event in lobby.gamepad_event_reader.iter(&gamepad_event) {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                lobby.gamepads.insert(*gamepad);
                println!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                lobby.gamepads.remove(gamepad);
                println!("{:?} Disconnected", gamepad);
            }
            _ => continue,
        }
    }
}
