use bevy::prelude::*;

mod components;
mod systems;

use crate::components::*;
use crate::systems::*;

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
        .add_system(animate_sprite_system.system())
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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_anim_handle = asset_server
        .load_sync(&mut textures, "assets/sprites/whisper.png")
        .unwrap();

    let texture = textures.get(&idle_anim_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(idle_anim_handle, texture.size, 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(6.75),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.225, true))
        .with(Velocity::zero())
        .with(Player::new(800.0));
}

fn player_movement(
    axes: Res<Axis<GamepadAxis>>,
    button_inputs: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    lobby: Res<GamepadLobby>,
    paused: Res<Paused>,
    time: Res<Time>,
    mut player_info: Query<(&Player, &mut Transform, &mut Velocity)>,
) {
    if paused.0 {
        return;
    }

    for (player, mut transform, mut velocity) in &mut player_info.iter() {
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
            // Tempoaray, remove when these buttons have other uses.
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
                *transform.translation_mut().x_mut() += 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
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
            let left_stick_y = axes
                .get(&GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();

            velocity.update_velocity(Vec2::new(left_stick_x, left_stick_y));
            let translation = velocity.0 * time.delta_seconds;

            *transform.translation_mut().x_mut() += player.move_speed * translation.x();
            *transform.translation_mut().y_mut() += player.move_speed * translation.y();
        }
    }
}
