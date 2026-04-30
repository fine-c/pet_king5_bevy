use bevy::{
    app::{App, Plugin},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec2, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;

const PLAYER_Z: f32 = 3.;
const PLAYER_SPEED: f32 = 83.3;
const PLAYER_COLOR: Color = Color::srgb(1.0, 0.8, 0.);
const PLAYER_SHAPE: Rectangle = Rectangle::new(40., 40.);

#[derive(Component)]
#[require(Position)]
pub struct Player;

pub struct PlayerPlugin {
    spawn_schedule: InternedScheduleLabel,
    input_schedule: InternedScheduleLabel,
}

impl PlayerPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel, input_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
            input_schedule: input_schedule.intern(),
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn_schedule, spawn_player)
            .add_systems(self.input_schedule, move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(PLAYER_SHAPE));
    let material = materials.add(ColorMaterial::from(PLAYER_COLOR));

    commands.spawn((
        Player,
        CameraTarget,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0., 0., PLAYER_Z),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut position: Single<&mut Position, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    direction = direction.normalize_or_zero();
    position.0 += direction * PLAYER_SPEED * time.delta_secs();
}
