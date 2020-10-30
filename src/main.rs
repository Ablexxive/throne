use bevy::prelude::*;

mod common;
mod gamepad_input;
mod pause_screen;

use common::*;
use gamepad_input::*;
use pause_screen::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Heaven Through Violence".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        .add_resource(Paused(false))
        .add_resource(GamepadLobby::default())
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_player")
        .add_startup_system_to_stage("spawn_player", spawn_player.system())
        .add_system(connection_system.system())
        .add_system(pause.system())
        .add_system(player_movement.system())
        .add_default_plugins()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());

    // Initial Resources
    commands.insert_resource(SpritePlaceholderMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));

    // Pause Menu Elements
    commands.spawn(UiCameraComponents::default());
    commands
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::Baseline,
                size: bevy::prelude::Size::new(Val::Px(200.0), Val::Px(200.0)),
                ..Default::default()
            },
            text: Text {
                value: "Pause".to_string(),
                font: asset_server.load("assets/fonts/SFNS.ttf").unwrap(),
                style: TextStyle {
                    font_size: 200.0,
                    color: Color::WHITE,
                },
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(PauseScreenItem);
}

struct Player;

fn spawn_player(mut commands: Commands, sprite_material: Res<SpritePlaceholderMaterial>) {
    commands
        .spawn(SpriteComponents {
            material: sprite_material.0.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Player);
}

fn player_movement(
    paused: Res<Paused>,
    keyboard_input: Res<Input<KeyCode>>,
    lobby: Res<GamepadLobby>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut player_position: Query<(&Player, &mut Transform)>,
) {
    if paused.0 {
        return;
    }

    for (_player, mut transform) in &mut player_position.iter() {
        // Check Keyboard input
        if keyboard_input.pressed(KeyCode::Left) {
            *transform.translation_mut().x_mut() -= 10.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            *transform.translation_mut().x_mut() += 10.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *transform.translation_mut().y_mut() -= 10.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *transform.translation_mut().y_mut() += 10.;
        }

        // Check Gamepad input
        for gamepad in lobby.gamepads.iter().cloned() {
            // TODO - Currently D-pad doesn't work for Bevy? Use these until you get the code to
            // move linked up to the sticks.
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
                *transform.translation_mut().x_mut() += 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
                eprintln!("Pressed!");
                *transform.translation_mut().x_mut() -= 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
                *transform.translation_mut().y_mut() += 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
                *transform.translation_mut().y_mut() -= 10.;
            }

            // Sample of code to read stick input.
            let left_stick_x = axes
                .get(&GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > 0.01 {
                println!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
            }
            let right_stick_x = axes
                .get(&GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if right_stick_x.abs() > 0.01 {
                println!("{:?} RightStickX value is {}", gamepad, right_stick_x);
            }
        }
    }
}
