use bevy::prelude::*;

pub struct ThroneCameraPlugin;

impl Plugin for ThroneCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_cameras.system())
            .add_startup_stage("setup_camera")
            .add_startup_system_to_stage("setup_camera", setup_camera.system());
        // add system to update camera on player
        //.add_system_to_stage(stage::POST_UPDATE, physics::sync_transform_system.system())
    }
}

pub struct PlayerCamera {
    pub zoom: f32,
}

//TODO(Sahil) - Load from config.
impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera { zoom: 0.15 }
    }
}

// Create a plugin that has two systems,
// one for spawning Camera and setting zoom,
// one for updating the camera to follow player. stage::UPDATE, POST_UPDATE?
// Maybe camera input controls here too?
pub fn spawn_cameras(mut commands: Commands) {
    // Player Camera
    commands
        .spawn(Camera2dComponents::default())
        .with(PlayerCamera::default());

    // UI Camera
    commands.spawn(UiCameraComponents::default());
}

pub fn setup_camera(mut cam_transforms: Query<(&mut Transform, &mut PlayerCamera)>) {
    for (mut transform, player_camera) in cam_transforms.iter_mut() {
        *transform.scale.x_mut() = player_camera.zoom;
        *transform.scale.y_mut() = player_camera.zoom;
    }
}
