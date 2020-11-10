use bevy::prelude::*;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::na::Vector2;
use bevy_rapier2d::render::RapierRenderPlugin;

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
        .add_startup_system(spawn_enemies.system())
        .add_startup_system(spawn_walls.system())
        .add_system(animate_sprite_system.system())
        .add_system(connection_system.system())
        .add_system(pause.system())
        .add_system(player_movement.system())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());

    rapier_config.gravity = Vector2::zeros();

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
    let scale_val = 6.75;
    let sprite_size_x = 16.0;
    let sprite_size_y = 23.0;

    let texture_atlas = TextureAtlas::from_grid(
        idle_anim_handle,
        Vec2::new(sprite_size_x, sprite_size_y),
        4,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(scale_val, scale_val, 1.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.225, true))
        .with(
            RigidBodyBuilder::new_dynamic()
                .can_sleep(false)
                .angular_damping(std::f32::INFINITY),
        )
        .with(ColliderBuilder::cuboid(
            (sprite_size_x / 2.0) * scale_val,
            (sprite_size_y / 2.0) * scale_val,
        ))
        .with(Player::new(800.0));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_anim_handle = asset_server.load("sprites/evil_whisper.png");

    let sprite_size_x = 16.0;
    let sprite_size_y = 23.0;
    let scale_val = 6.75;

    let texture_atlas = TextureAtlas::from_grid(
        idle_anim_handle,
        Vec2::new(sprite_size_x, sprite_size_y),
        4,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();
    for enemy_idx in 1..=3 {
        let anim_timer = rng.gen_range(0.175, 0.300);
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    scale: Vec3::new(scale_val, scale_val, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(anim_timer, true))
            .with(
                RigidBodyBuilder::new_dynamic()
                    .translation(((enemy_idx as f32) * 150.0) - 300.0, 300.0)
                    .angular_damping(std::f32::INFINITY)
                    .linear_damping(10.0),
            )
            .with(ColliderBuilder::cuboid(
                (sprite_size_x / 2.0) * scale_val,
                (sprite_size_y / 2.0) * scale_val,
            ));
    }
}

fn spawn_walls(mut commands: Commands) {
    let wall_side = 16.0;
    let scale_val = 6.75;

    for wall_idx in 1..=3 {
        commands
            .spawn((Transform::default(),))
            .with(
                RigidBodyBuilder::new_kinematic()
                    .translation(((wall_idx as f32) * 150.0) - 300.0, -300.0),
            )
            .with(ColliderBuilder::cuboid(
                (wall_side / 2.0) * scale_val,
                (wall_side / 2.0) * scale_val,
            ));
    }
}
fn player_movement(
    axes: Res<Axis<GamepadAxis>>,
    keyboard_input: Res<Input<KeyCode>>,
    lobby: Res<GamepadLobby>,
    paused: Res<Paused>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    player_info: Query<(&Player, &RigidBodyHandleComponent)>,
) {
    if paused.0 {
        return;
    }

    for (player, rigid_body_component) in player_info.iter() {
        // First check Gamepad input
        if let Some(gamepad) = lobby.gamepads.iter().cloned().next() {
            // TODO(Sahil) - Tempoaray reference, remove when these buttons have other uses.
            //button_inputs: Res<Input<GamepadButton>>,
            //if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::East)) {}
            let left_stick_x = axes
                .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            let left_stick_y = axes
                .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();

            let move_delta = Vector2::new(left_stick_x, left_stick_y);
            let move_delta = if move_delta == Vector2::zeros() || move_delta.magnitude() < 0.1 {
                Vector2::zeros()
            } else {
                move_delta
            };

            if let Some(mut rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
                rb.linvel = move_delta * player.move_speed;
            }
        } else {
            // Check Keyboard input
            let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8)
                + (keyboard_input.pressed(KeyCode::D) as i8);
            let y_axis = -(keyboard_input.pressed(KeyCode::S) as i8)
                + (keyboard_input.pressed(KeyCode::W) as i8);
            let mut move_delta = Vector2::new(x_axis as f32, y_axis as f32);
            if move_delta != Vector2::zeros() {
                move_delta /= move_delta.magnitude();
            }
            if let Some(mut rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
                rb.linvel = move_delta * player.move_speed;
            }
        }
    }
}
