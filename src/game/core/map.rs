use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct MapSize {
    pub width: i32,
    pub height: i32,
}
