use bevy::app::startup_stage::PRE_STARTUP;
use bevy::prelude::*;

use crate::Player;

pub struct ThroneCameraPlugin;

/// Plugin to spawn camera, set it up, and update it to center on player.
impl Plugin for ThroneCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(PRE_STARTUP, spawn_cameras.system())
            .add_startup_system(setup_camera.system())
            .add_system_to_stage(stage::POST_UPDATE, update_camera.system());
    }
}

pub struct PlayerCamera {
    pub scale_factor: f32,
}

//TODO(Sahil) - Load from config.
impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera { scale_factor: 0.15 }
    }
}

fn spawn_cameras(commands: &mut Commands) {
    // Player Camera
    commands
        .spawn(Camera2dBundle::default())
        .with(PlayerCamera::default());

    //UI Camera
    commands.spawn(CameraUiBundle::default());
}

fn setup_camera(mut cam_transforms: Query<(&mut Transform, &mut PlayerCamera)>) {
    for (mut transform, player_camera) in cam_transforms.iter_mut() {
        transform.scale.x = player_camera.scale_factor;
        transform.scale.y = player_camera.scale_factor;
    }
}

fn update_camera(
    player_transforms: Query<(&Transform, &Player)>,
    mut cam_transforms: Query<(&mut Transform, &mut PlayerCamera)>,
) {
    if let Some((player_transform, _player)) = player_transforms.iter().next() {
        for (mut camera_transform, _player_camera) in cam_transforms.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
