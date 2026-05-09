use bevy::{
    DefaultPlugins,
    app::{App, Plugin, PluginGroup},
    image::ImagePlugin,
    utils::default,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin},
};
use bevy_aseprite_ultra::AsepriteUltraPlugin;

pub struct AppBootstrapPlugin;

impl Plugin for AppBootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AsepriteUltraPlugin);
    }
}
