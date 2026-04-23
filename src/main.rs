use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    camera::Camera2d,
    ecs::system::Commands,
    image::ImagePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
