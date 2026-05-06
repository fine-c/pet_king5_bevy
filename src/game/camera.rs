use bevy::{
    app::{App, Plugin, PostUpdate},
    camera::{Camera2d, OrthographicProjection, Projection, ScalingMode},
    ecs::{
        component::Component,
        query::{With, Without},
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Single},
    },
};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;

#[derive(Component)]
#[require(Camera2d, Position)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraAnchored;

pub struct CameraPlugin {
    spawn_schedule: InternedScheduleLabel,
}

impl CameraPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn_schedule, spawn_camera)
            .add_systems(PostUpdate, apply_camera_target_position);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 360.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn apply_camera_target_position(
    target_pos: Single<&Position, (With<CameraTarget>, Without<MainCamera>)>,
    mut camera_pos: Single<&mut Position, (With<MainCamera>, Without<CameraAnchored>)>,
) {
    camera_pos.0 = target_pos.0;
}
