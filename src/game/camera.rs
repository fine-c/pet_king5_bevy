use bevy::{
    app::{App, Plugin},
    camera::Camera2d,
    ecs::{
        component::Component,
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::Commands,
    },
};

use crate::engine::position::Position;

#[derive(Component)]
#[require(Camera2d, Position)]
pub struct MainCamera;

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
        app.add_systems(self.spawn_schedule, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Position::new(640.0 / 2.0, 640.0 / 2.0)));
}
