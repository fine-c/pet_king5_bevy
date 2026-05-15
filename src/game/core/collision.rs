use bevy::{ecs::component::Component, math::Rect};

#[derive(Component)]
pub struct CollisionRect(pub Rect);
