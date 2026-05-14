use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        schedule::IntoScheduleConfigs,
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    sprite::Sprite,
    time::Time,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;
use crate::game::core::player_spawn::PlayerMarker;

const DEFAULT_PLAYER_SPEED: f32 = 83.3;

#[derive(Debug, Default, PartialEq)]
enum PlayerState {
    #[default]
    Stand,
    Walk,
}
#[derive(Debug, Default, PartialEq)]
enum PlayerDirection {
    Left,
    Up,
    Right,
    #[default]
    Down,
}

#[derive(Component, Debug)]
#[require(Position)]
struct Player {
    direction: PlayerDirection,
    state: PlayerState,
    walk_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: PlayerDirection::default(),
            state: PlayerState::default(),
            walk_speed: DEFAULT_PLAYER_SPEED,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_player, (control_player, player_animation).chain()),
        );
    }
}

fn setup_player(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    entity: Single<Entity, Added<PlayerMarker>>,
) {
    commands.entity(*entity).insert((
        AseAnimation {
            animation: Animation::tag("idle_down"),
            aseprite: assets_server.load("player/0.aseprite"),
        },
        CameraTarget,
        Player::default(),
        Sprite::default(),
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
