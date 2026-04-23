use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    asset::AssetServer,
    camera::{Camera2d, OrthographicProjection, Projection},
    ecs::system::{Commands, Res},
    image::ImagePlugin,
    transform::components::Transform,
};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(640.0 / 2.0, 640.0 / 2.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("pet_king5.ldtk").into(),
        ..Default::default()
    });
}
