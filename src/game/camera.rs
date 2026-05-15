use bevy::{
    app::{App, Plugin, PostUpdate},
    camera::{Camera2d, OrthographicProjection, Projection, ScalingMode},
    ecs::{
        component::Component,
        query::{With, Without},
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Res, Single},
    },
};

use crate::engine::position::Position;
use crate::game::core::camera_target::CameraTarget;
use crate::game::core::map::MapSize;

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
    mut camera_pos: Single<&mut Position, (With<MainCamera>, Without<CameraAnchored>)>,
    projection: Single<&Projection, With<MainCamera>>,
    target_pos: Single<&Position, (With<CameraTarget>, Without<MainCamera>)>,
    map_size: Option<Res<MapSize>>,
) {
    let Some(map_size) = map_size else {
        return;
    };

    let Projection::Orthographic(ortho) = *projection else {
        return;
    };

    let area = ortho.area;
    let min_x = area.width() / 2.;
    let min_y = area.height() / 2.;
    let max_x = (map_size.width as f32 - min_x).max(min_x);
    let max_y = (map_size.height as f32 - min_y).max(min_y);

    camera_pos.0.x = target_pos.0.x.clamp(min_x, max_x);
    camera_pos.0.y = target_pos.0.y.clamp(min_y, max_y);
}
