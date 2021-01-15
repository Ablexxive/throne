use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::na::Vector2;

use rand::Rng;
use ron;
use serde::Deserialize;
use std::fs;

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
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(ThroneCameraPlugin)
        .add_resource(GamepadLobby::default())
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_entities")
        .add_startup_system_to_stage("spawn_entities", spawn_player.system())
        .add_startup_system_to_stage("spawn_entities", spawn_enemies.system())
        .add_startup_system_to_stage("spawn_entities", spawn_walls.system())
        //.add_system_to_stage_front(stage::PRE_UPDATE, remove_rotation.system())
        .add_system(scene_management.thread_local_system())
        .add_system(animate_sprite_system.system())
        .add_system(connection_system.system())
        .add_system(player_movement.system())
        .add_system_to_stage(stage::POST_UPDATE, debug_ui_update.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(TextComponents {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            value: "Player Pos: -0.1234567890".to_string(),
            font: asset_server.load("fonts/SFNS.ttf"),
            style: TextStyle {
                font_size: 70.0,
                color: Color::BLACK,
            },
        },
        ..Default::default()
    });

    rapier_config.gravity = Vector2::zeros();

    // Initial Resources
    commands.insert_resource(SpritePlaceholderMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    eprintln!("Spawning player.");
    let idle_anim_handle = asset_server.load("sprites/whisper.png");

    // TODO(Sahil) - The textures here are loaded async in the background, so you can't yet access
    // `texture.size`. Might be worth generating some sort of metadata file to hold that
    // information.
    let scale_val = 1.0;
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
            RigidBodyBuilder::new_dynamic().can_sleep(false), //.angular_damping(std::f32::INFINITY),
        )
        .with(ColliderBuilder::cuboid(
            (sprite_size_x / 2.0) * scale_val,
            (sprite_size_y / 2.0) * scale_val,
        ))
        .with(LockRotation)
        .with(Player::new(100.0));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    eprintln!("Spawning enemies.");
    let idle_anim_handle = asset_server.load("sprites/evil_whisper.png");

    let sprite_size_x = 16.0;
    let sprite_size_y = 23.0;
    let scale_val = 1.0;

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
                    //.angular_damping(std::f32::INFINITY)
                    //.linear_damping(10.0),
            )
            .with(LockRotation)
            .with(ColliderBuilder::cuboid(
                (sprite_size_x / 2.0) * scale_val,
                (sprite_size_y / 2.0) * scale_val,
            ));
    }
}

#[derive(Deserialize, Debug)]
struct Wall {
    idx: u32,
    x: f32,
    y: f32,
    height: f32,
    width: f32,
}

#[derive(Deserialize, Debug)]
struct Walls {
    walls: Vec<Wall>,
}

fn spawn_wall(
    wall_x: f32,
    wall_y: f32,
    wall_width: f32,
    wall_height: f32,
    commands: &mut Commands,
    wall_material: &Res<SpritePlaceholderMaterial>,
) {
    // Sprites spawn with their translation specifying the center of the sprite.
    // We want the bottom left corner of the wall to be at the input (wall_x, wall_y)
    let updated_wall_x = wall_x + (0.5 * wall_width);
    let updated_wall_y = wall_y + (0.5 * wall_height);
    commands
        .spawn(SpriteComponents {
            material: wall_material.0.clone(),
            sprite: Sprite::new(Vec2::new(wall_width, wall_height)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(updated_wall_x, updated_wall_y))
        .with(ColliderBuilder::cuboid(wall_width / 2.0, wall_height / 2.0));
}

// TODO(Sahil) - refactor out and rename.
fn spawn_walls(mut commands: Commands, wall_material: Res<SpritePlaceholderMaterial>) {
    let wall_definition = fs::read_to_string("wall_definition.ron").unwrap();
    let walls: Walls = ron::de::from_str(&wall_definition).unwrap();

    eprintln!("Spawning outside walls.");
    //let unscaled_wall_width = 16.0;
    //let unscaled_wall_length = 16.0;
    //let scale_val = 1.0;
    //let wall_width = unscaled_wall_width * scale_val;
    //let wall_length = unscaled_wall_length * scale_val;

    //for wall_idx in 1..=3 {
    for wall in walls.walls {
        spawn_wall(
            wall.x,
            wall.y,
            wall.width,
            wall.height,
            &mut commands,
            &wall_material,
        );
    }
}

fn player_movement(
    mut commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    button_inputs: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    lobby: Res<GamepadLobby>,
    asset_server: Res<AssetServer>,
    wall_material: Res<SpritePlaceholderMaterial>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    player_info: Query<(&Player, &Transform, &RigidBodyHandleComponent)>,
    mut camera_info: Query<(&mut Camera, &mut Transform, &mut PlayerCamera)>,
) {
    for (player, player_transform, rigid_body_component) in player_info.iter() {
        // First check Gamepad input
        if let Some(gamepad) = lobby.gamepads.iter().cloned().next() {
            // TODO(Sahil) - Have this shoot off an event which spawns enemies instead of doing it
            // like this.
            if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
                let wall_side = 16.0;
                let scale_val = 1.0;

                commands
                    .spawn(SpriteComponents {
                        material: wall_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(
                            wall_side * scale_val,
                            wall_side * scale_val,
                        )),
                        ..Default::default()
                    })
                    .with(RigidBodyBuilder::new_dynamic().translation(
                        player_transform.translation.x(),
                        player_transform.translation.y(),
                    ))
                    .with(ColliderBuilder::cuboid(
                        (wall_side / 2.0) * scale_val,
                        (wall_side / 2.0) * scale_val,
                    ));
            }
            if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
                let idle_anim_handle = asset_server.load("sprites/evil_whisper.png");

                let sprite_size_x = 16.0;
                let sprite_size_y = 23.0;
                let scale_val = 1.0;

                let texture_atlas = TextureAtlas::from_grid(
                    idle_anim_handle,
                    Vec2::new(sprite_size_x, sprite_size_y),
                    4,
                    1,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let mut rng = rand::thread_rng();
                let anim_timer = rng.gen_range(0.175, 0.300);
                commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            scale: Vec3::new(scale_val, scale_val, 1.0),
                            translation: Vec3::new(
                                player_transform.translation.x(),
                                player_transform.translation.y(),
                                0.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(Timer::from_seconds(anim_timer, true))
                    .with(
                        RigidBodyBuilder::new_dynamic()
                            //.translation(300.0, 500.0)
                            .translation(
                                player_transform.translation.x(),
                                player_transform.translation.y(),
                            )
                            //.angular_damping(std::f32::INFINITY)
                            //.linear_damping(10.0),
                    )
                    .with(LockRotation)
                    .with(ColliderBuilder::cuboid(
                        (sprite_size_x / 2.0) * scale_val,
                        (sprite_size_y / 2.0) * scale_val,
                    ));
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::LeftTrigger)) {
                for (_camera, mut transform, _player_camera) in camera_info.iter_mut() {
                    *transform.scale.x_mut() += 0.01;
                    *transform.scale.y_mut() += 0.01;
                    eprintln!("Camera Scale: {}", transform.scale);
                }
            }
            if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::RightTrigger)) {
                for (_camera, mut transform, _player_camera) in camera_info.iter_mut() {
                    *transform.scale.x_mut() -= 0.01;
                    *transform.scale.y_mut() -= 0.01;
                    eprintln!("Camera Scale: {}", transform.scale);
                }
            }

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
