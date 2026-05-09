use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        schedule::{InternedScheduleLabel, IntoScheduleConfigs, ScheduleLabel},
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;

const PLAYER_Z: f32 = 3.;
const DEFAULT_PLAYER_SPEED: f32 = 83.3;

#[derive(Debug, PartialEq)]
enum PlayerState {
    Walk,
    Stand,
}
#[derive(Debug, PartialEq)]
enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
#[require(Position)]
struct Player {
    walk_speed: f32,
    state: PlayerState,
    direction: PlayerDirection,
}

pub struct PlayerPlugin {
    spawn_schedule: InternedScheduleLabel,
}

impl PlayerPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn_schedule, spawn_player)
            .add_systems(Update, (control_player, player_animation).chain());
    }
}

fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        AseAnimation {
            aseprite: assets_server.load("player/0.aseprite"),
            animation: Animation::tag("idle_down"),
        },
        CameraTarget,
        Player {
            walk_speed: DEFAULT_PLAYER_SPEED,
            state: PlayerState::Stand,
            direction: PlayerDirection::Down,
        },
        Sprite::default(),
        Transform::from_xyz(0., 0., PLAYER_Z),
    ));
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Single<(&mut Player, &mut Position)>,
) {
    let (player, position) = &mut *query;

    if input.just_pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::KeyA) {
        player.direction = PlayerDirection::Left;
    } else if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyW) {
        player.direction = PlayerDirection::Up;
    } else if input.just_pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::KeyD) {
        player.direction = PlayerDirection::Right;
    } else if input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyS) {
        player.direction = PlayerDirection::Down;
    }

    let direction = if player.direction == PlayerDirection::Left
        && (input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA))
    {
        Vec2::new(-1., 0.)
    } else if player.direction == PlayerDirection::Up
        && (input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW))
    {
        Vec2::new(0., 1.)
    } else if player.direction == PlayerDirection::Right
        && (input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD))
    {
        Vec2::new(1., 0.)
    } else if player.direction == PlayerDirection::Down
        && (input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS))
    {
        Vec2::new(0., -1.)
    } else if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        player.direction = PlayerDirection::Left;
        Vec2::new(-1., 0.)
    } else if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        player.direction = PlayerDirection::Up;
        Vec2::new(0., 1.)
    } else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        player.direction = PlayerDirection::Right;
        Vec2::new(1., 0.)
    } else if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        player.direction = PlayerDirection::Down;
        Vec2::new(0., -1.)
    } else {
        Vec2::ZERO
    };

    player.state = if direction == Vec2::ZERO {
        PlayerState::Stand
    } else {
        PlayerState::Walk
    };
    position.0 += direction * player.walk_speed * time.delta_secs();
}

fn player_animation(mut query: Single<(&mut AseAnimation, &Player)>) {
    let (ase_animation, player) = &mut *query;
    match player.state {
        PlayerState::Stand => match player.direction {
            PlayerDirection::Left => ase_animation.animation.play_loop("idle_left"),
            PlayerDirection::Up => ase_animation.animation.play_loop("idle_up"),
            PlayerDirection::Right => ase_animation.animation.play_loop("idle_right"),
            PlayerDirection::Down => ase_animation.animation.play_loop("idle_down"),
        },
        PlayerState::Walk => match player.direction {
            PlayerDirection::Left => ase_animation.animation.play_loop("walk_left"),
            PlayerDirection::Up => ase_animation.animation.play_loop("walk_up"),
            PlayerDirection::Right => ase_animation.animation.play_loop("walk_right"),
            PlayerDirection::Down => ase_animation.animation.play_loop("walk_down"),
        },
    }
}
