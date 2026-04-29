use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::Commands,
    },
};

#[derive(Component)]
pub struct Cream;

pub struct CreamPlugin {
    spawn_schedule: InternedScheduleLabel,
}

impl CreamPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
        }
    }
}

impl Plugin for CreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn_schedule, spawn_cream);
    }
}

fn spawn_cream(mut commands: Commands) {
    commands.spawn(Cream);
}
