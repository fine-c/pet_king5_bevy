use bevy::{
    app::{App, Plugin, PostUpdate},
    ecs::{component::Component, system::Query},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, apply_position);
    }
}

fn apply_position(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}
