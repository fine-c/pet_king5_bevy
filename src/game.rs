use bevy::{
    app::{App, Plugin},
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
};

use crate::game::{camera::CameraPlugin, player::PlayerPlugin, world::WorldPlugin};

pub mod camera;
pub mod core;
pub mod player;
pub mod world;

pub struct GamePlugin {
    world_spawn: InternedScheduleLabel,
    player_spawn: InternedScheduleLabel,
    camera_spawn: InternedScheduleLabel,
}

impl GamePlugin {
    pub fn new(
        world_spawn: impl ScheduleLabel,
        player_spawn: impl ScheduleLabel,
        camera_spawn: impl ScheduleLabel,
    ) -> Self {
        Self {
            world_spawn: world_spawn.intern(),
            player_spawn: player_spawn.intern(),
            camera_spawn: camera_spawn.intern(),
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldPlugin::new(self.world_spawn),
            CameraPlugin::new(self.camera_spawn),
            PlayerPlugin::new(self.player_spawn),
        ));
    }
}
