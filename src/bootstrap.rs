use bevy::{
    DefaultPlugins,
    app::{App, Plugin, PluginGroup},
    image::ImagePlugin,
};

pub struct AppBootstrapPlugin;

impl Plugin for AppBootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    }
}
