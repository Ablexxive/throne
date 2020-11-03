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

pub struct Player {
    pub move_speed: f32,
}

impl Player {
    pub fn new(move_speed: f32) -> Self {
        Self { move_speed }
    }
}

#[derive(Clone, Debug)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn update_velocity(&mut self, new_velocity: Vec2) {
        self.0 = new_velocity;
    }
}

impl From<Vec2> for Velocity {
    fn from(val: Vec2) -> Self {
        Self(val)
    }
}

fn spawn_player(mut commands: Commands, sprite_material: Res<SpritePlaceholderMaterial>) {
    commands
        .spawn(SpriteComponents {
            material: sprite_material.0.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
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
