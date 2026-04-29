use bevy::{
    app::{App, Plugin},
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
};

use crate::game::{cream::CreamPlugin, player::PlayerPlugin, world::WorldPlugin};

pub mod core;
pub mod cream;
pub mod player;
pub mod world;

pub struct GamePlugin {
    world_spawn: InternedScheduleLabel,
    player_spawn: InternedScheduleLabel,
    player_input: InternedScheduleLabel,
    cream_spawn: InternedScheduleLabel,
}

impl GamePlugin {
    pub fn new(
        world_spawn: impl ScheduleLabel,
        player_spawn: impl ScheduleLabel,
        player_input: impl ScheduleLabel,
        cream_spawn: impl ScheduleLabel,
    ) -> Self {
        Self {
            world_spawn: world_spawn.intern(),
            player_spawn: player_spawn.intern(),
            player_input: player_input.intern(),
            cream_spawn: cream_spawn.intern(),
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldPlugin::new(self.world_spawn),
            CreamPlugin::new(self.cream_spawn),
            PlayerPlugin::new(self.player_spawn, self.player_input),
        ));
    }
}
