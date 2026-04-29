use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    camera::{Camera2d, OrthographicProjection, Projection},
    ecs::{
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Res},
    },
    transform::components::Transform,
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

pub struct WorldPlugin {
    spawn_schedule: InternedScheduleLabel,
}

impl WorldPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(self.spawn_schedule, setup_world);
    }
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(640.0 / 2.0, 640.0 / 2.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("pet_king5.ldtk").into(),
        ..Default::default()
    });
}
