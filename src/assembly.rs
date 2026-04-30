use bevy::{
    app::{App, Plugin, Update},
    state::{app::AppExtStates, state::OnEnter},
};

use crate::game::core::app_state::AppState;
use crate::{engine::EnginePlugin, game::GamePlugin};

pub struct GameAssemblyPlugin;

impl Plugin for GameAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnginePlugin,
            GamePlugin::new(
                OnEnter(AppState::InGame),
                OnEnter(AppState::InGame),
                Update,
                OnEnter(AppState::InGame),
            ),
        ))
        .insert_state(AppState::InGame);
    }
}
