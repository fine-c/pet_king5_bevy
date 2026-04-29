use bevy::app::App;
use pet_king5_bevy::assembly::GameAssemblyPlugin;
use pet_king5_bevy::bootstrap::AppBootstrapPlugin;

fn main() {
    App::new()
        .add_plugins((AppBootstrapPlugin, GameAssemblyPlugin))
        .run();
}
