use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    ecs::{
        bundle::Bundle,
        message::MessageReader,
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Res, Single},
    },
};
use bevy_ecs_ldtk::{
    LdtkEntity, LdtkPlugin, LdtkProjectHandle, LdtkWorldBundle, LevelEvent, LevelSelection,
    app::LdtkEntityAppExt,
    assets::{LdtkProject, LevelMetadataAccessor},
};

use crate::game::core::map::MapSize;
use crate::game::core::player_spawn::PlayerMarker;

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
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(self.spawn_schedule, setup_world)
            .add_systems(Update, apply_map_size);
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    marker: PlayerMarker,
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("pet_king5.ldtk").into(),
        ..Default::default()
    });
}

fn apply_map_size(
    mut commands: Commands,
    mut level_events: MessageReader<LevelEvent>,
    project_handles: Single<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    for event in level_events.read() {
        let LevelEvent::Transformed(level_iid) = event else {
            continue;
        };

        let Some(level) = project_assets
            .get(*project_handles)
            .and_then(|p| p.get_raw_level_by_iid(&level_iid.to_string()))
        else {
            continue;
        };

        commands.insert_resource(MapSize {
            width: level.px_wid,
            height: level.px_hei,
        });
    }
}
