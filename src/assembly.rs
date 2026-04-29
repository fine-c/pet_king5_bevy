use bevy::{
    app::{App, Plugin, Update},
    state::{app::AppExtStates, state::OnEnter},
};

use crate::game::GamePlugin;
use crate::game::core::AppState;

pub struct GameAssemblyPlugin;

impl Plugin for GameAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePlugin::new(
            OnEnter(AppState::InGame),
            OnEnter(AppState::InGame),
            Update,
            OnEnter(AppState::InGame),
        ))
        .insert_state(AppState::InGame);
    }
}
