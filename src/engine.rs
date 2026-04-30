use bevy::app::{App, Plugin};

use crate::engine::position::PositionPlugin;

pub mod position;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PositionPlugin);
    }
}
