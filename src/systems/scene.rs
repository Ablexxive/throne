use std::fs;

use bevy::{prelude::*, reflect::TypeRegistry};

use crate::GamepadLobby;

pub fn load_scene(asset_server: &AssetServer, scene_spawner: &mut SceneSpawner) {
    eprintln!("Loading scene.");
    // Scenes are loaded just like any other asset.
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/scene_test.scn");

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in the World with new entity ids.
    // This guarantees that it will not overwrite existing entities.
    scene_spawner.spawn_dynamic(scene_handle);

    // This tells the AssetServer to watch for changes to assets.
    // It enables our scenes to automatically reload in game when we modify their files
    asset_server.watch_for_changes().unwrap();
}

pub fn save_scene(world: &mut World, type_registry: &TypeRegistry) {
    let scene = DynamicScene::from_world(&world, &type_registry);
    let serialized = scene.serialize_ron(&type_registry).unwrap();
    fs::write("scene_test.scn", serialized).unwrap();
}

pub fn scene_management(world: &mut World, resources: &mut Resources) {
    let lobby = resources.get::<GamepadLobby>().unwrap();
    let button_inputs = resources.get::<Input<GamepadButton>>().unwrap();

    if let Some(gamepad) = lobby.gamepads.iter().cloned().next() {
        // Save the scene
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::Select)) {
            let type_registry = resources.get::<TypeRegistry>().unwrap();

            eprintln!("Saving scene!");
            save_scene(world, &type_registry);
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::Start)) {
            let asset_server = resources.get_mut::<AssetServer>().unwrap();
            let mut scene_spawner = resources.get_mut::<SceneSpawner>().unwrap();
            load_scene(&asset_server, &mut scene_spawner);
        }
    }
}
