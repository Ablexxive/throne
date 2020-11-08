use bevy::prelude::*;
use bevy_rapier2d::physics::{RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::na::{coordinates::XY, Isometry2, Vector2};

// Rapier Notes -
// RapierPhysicsPlugin - will register a system for building bodies, perform one timestep, and
// write ridgid-bodies postions back into translation and rotation components

use rand::Rng;

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
        .add_startup_system(spawn_player.system())
        //.add_startup_system(spawn_enemies.system())
        .add_system(animate_sprite_system.system())
        .add_system(connection_system.system())
        .add_system(pause.system())
        .add_system(player_movement.system())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
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
                //font: asset_server.load("assets/fonts/SFNS.ttf"),
                font: asset_server.load("fonts/SFNS.ttf"),
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_anim_handle = asset_server.load("sprites/whisper.png");

    // TODO(Sahil) - The textures here are loaded async in the background, so you can't yet access
    // `texture.size`. Might be worth generating some sort of metadata file to hold that
    // information.
    let texture_atlas = TextureAtlas::from_grid(idle_anim_handle, Vec2::new(32.0, 32.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(6.75, 6.75, 1.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.225, true))
        .with(Velocity::zero())
        .with(RigidBodyBuilder::new_dynamic().can_sleep(false))
        .with(ColliderBuilder::cuboid(32.0, 32.0))
        .with(Player::new(800.0));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_anim_handle = asset_server.load("sprites/evil_whisper.png");

    let texture_atlas = TextureAtlas::from_grid(idle_anim_handle, Vec2::new(32.0, 32.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();
    for enemy_idx in 1..=3 {
        let anim_timer = rng.gen_range(0.175, 0.300);
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    scale: Vec3::new(6.75, 6.75, 1.0),
                    translation: Vec3::new(((enemy_idx as f32) * 150.0) - 300.0, 300.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(anim_timer, true))
            .with(Velocity::zero())
            .with(RigidBodyBuilder::new_static())
            .with(ColliderBuilder::cuboid(32.0 * 6.75, 1.0));
    }
}

fn player_movement(
    axes: Res<Axis<GamepadAxis>>,
    button_inputs: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    lobby: Res<GamepadLobby>,
    paused: Res<Paused>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    //mut player_transform: Query<(&Player, &mut Transform, &mut Velocity)>,
    mut player_info: Query<(
        &Player,
        &mut RigidBodyHandleComponent,
        &mut Velocity,
        &mut Transform,
    )>,
) {
    if paused.0 {
        return;
    }
    //for (player, mut transform, mut velocity) in player_info.iter_mut() {
    for (player, mut rigid_body_component, mut velocity, mut transform) in player_info.iter_mut() {
        // Check Keyboard input
        if keyboard_input.pressed(KeyCode::Left) {
            //*transform.translation.x_mut() -= 10.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            //*transform.translation.x_mut() += 10.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            //*transform.translation.y_mut() -= 10.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            //*transform.translation.y_mut() += 10.;
        }

        // Check Gamepad input
        for gamepad in lobby.gamepads.iter().cloned() {
            // Tempoaray, remove when these buttons have other uses.
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
                //*transform.translation.x_mut() += 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
                //*transform.translation.x_mut() -= 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
                //*transform.translation.y_mut() += 10.;
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
                //*transform.translation.y_mut() -= 10.;
            }

            // Sample of code to read stick input.
            let left_stick_x = axes
                .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            let left_stick_y = axes
                .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();

            //velocity.update_velocity(Vec2::new(left_stick_x, left_stick_y));
            //dbg!(&velocity);
            if let Some(mut rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
                //rb.set_position(Isometry2::new(Vector2::new(300.0, 300.0), 0.0));
                //*rb.linvel = XY {
                //x: left_stick_x * player.move_speed,
                //y: left_stick_y * player.move_speed,
                //};
                rb.linvel = Vector2::new(
                    (left_stick_x * player.move_speed),
                    (left_stick_y * player.move_speed),
                );
                //dbg!(&rb.linvel);
                //dbg!(&rb.position);
                //dbg!(&transform);
                //*rb.linvel = Vector2::new(velocity.x(), velocity.y());
            }
            //let translation = velocity.0 * time.delta_seconds;

            //*transform.translation.x_mut() += player.move_speed * translation.x();
            //*transform.translation.y_mut() += player.move_speed * translation.y();
        }
    }
}
