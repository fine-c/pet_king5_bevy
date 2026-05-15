use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        query::Added,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Rect, Vec2},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;
use crate::game::core::collision::CollisionRect;
use crate::game::core::player_spawn::PlayerMarker;

const DEFAULT_PLAYER_SPEED: f32 = 83.3;
const PLAYER_HALF_SIZE: Vec2 = Vec2::new(9.5, 9.5);
const SPRITE_OFFSET_Y: f32 = 11.0;

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
    commands
        .entity(*entity)
        .insert((CameraTarget, Player::default()))
        .with_children(|parent| {
            parent.spawn((
                AseAnimation {
                    animation: Animation::tag("idle_down"),
                    aseprite: assets_server.load("player/0.aseprite"),
                },
                Sprite::default(),
                Transform::from_xyz(0.0, SPRITE_OFFSET_Y, 0.0),
            ));
        });
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<(&mut Player, &mut Position)>,
    collision_rects: Query<&CollisionRect>,
) {
    let (player, position) = &mut *player;

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

    let movement = direction * player.walk_speed * time.delta_secs();
    position.0 = apply_collision(position.0, movement, &collision_rects);
}

fn apply_collision(pos: Vec2, movement: Vec2, rects: &Query<&CollisionRect>) -> Vec2 {
    let target_x =
        Rect::from_center_half_size(Vec2::new(pos.x + movement.x, pos.y), PLAYER_HALF_SIZE);
    let x = if rects.iter().any(|r| intersects(target_x, r.0)) {
        pos.x
    } else {
        pos.x + movement.x
    };

    let target_y = Rect::from_center_half_size(Vec2::new(x, pos.y + movement.y), PLAYER_HALF_SIZE);
    let y = if rects.iter().any(|r| intersects(target_y, r.0)) {
        pos.y
    } else {
        pos.y + movement.y
    };

    Vec2::new(x, y)
}

fn intersects(a: Rect, b: Rect) -> bool {
    let x = a.min.x < b.max.x && a.max.x > b.min.x;
    let y = a.min.y < b.max.y && a.max.y > b.min.y;
    x && y
}

fn player_animation(player: Single<(&Children, &Player)>, mut query: Query<&mut AseAnimation>) {
    let (children, player) = &*player;
    for child in children.iter() {
        let Ok(mut ase_animation) = query.get_mut(*child) else {
            continue;
        };

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
}
