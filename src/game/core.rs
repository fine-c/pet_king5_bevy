use bevy::state::state::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}
