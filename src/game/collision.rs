use std::collections::HashSet;

use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        query::{Added, Without},
        system::{Commands, Query, Res},
    },
    math::{Rect, Vec2},
    platform::collections::HashMap,
    transform::components::Transform,
};
use bevy_ecs_ldtk::{
    GridCoords, LdtkIntCell, LdtkProjectHandle, LevelIid, app::LdtkIntCellAppExt,
    assets::LdtkProject, ldtk::LayerInstance,
};

use crate::game::core::collision::CollisionRect;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Plate {
    left: i32,
    right: i32,
}

#[derive(Component, Default)]
struct Wall;

#[derive(Bundle, Default, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

const WALL: i32 = 1;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(WALL)
            .add_systems(Update, spawn_wall_collision);
    }
}

fn spawn_wall_collision(
    mut commands: Commands,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_query: Query<(Entity, &LevelIid)>,
    parent_query: Query<&ChildOf, Without<Wall>>,
    wall_query: Query<(&GridCoords, &ChildOf), Added<Wall>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let mut level_to_walls: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    for (&grid_coords, child_of) in &wall_query {
        if let Ok(parent) = parent_query.get(child_of.parent()) {
            level_to_walls
                .entry(parent.parent())
                .or_default()
                .insert(grid_coords);
        }
    }

    if wall_query.is_empty() {
        return;
    }

    for (level_entity, level_iid) in &level_query {
        let Some(level_walls) = level_to_walls.remove(&level_entity) else {
            continue;
        };

        let Some(ldtk_project) = ldtk_projects
            .iter()
            .next()
            .and_then(|h| ldtk_project_assets.get(h))
        else {
            continue;
        };

        let Some(level) = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
        else {
            continue;
        };

        let LayerInstance {
            c_wid: width,
            c_hei: height,
            grid_size,
            ..
        } = level.layer_instances()[0];

        let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

        for y in 0..height {
            let mut row_plates = Vec::new();
            let mut start: Option<i32> = None;

            for x in 0..width + 1 {
                match (start, level_walls.contains(&GridCoords { x, y })) {
                    (Some(s), false) => {
                        row_plates.push(Plate {
                            left: s,
                            right: x - 1,
                        });
                        start = None;
                    }
                    (None, true) => start = Some(x),
                    _ => {}
                }
            }

            plate_stack.push(row_plates);
        }

        let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
        let mut prev_row: Vec<Plate> = Vec::new();
        let mut wall_rects: Vec<Rect> = Vec::new();

        plate_stack.push(Vec::new());

        for (y, current_row) in plate_stack.into_iter().enumerate() {
            for prev_plate in &prev_row {
                if !current_row.contains(prev_plate)
                    && let Some(rect) = rect_builder.remove(prev_plate)
                {
                    wall_rects.push(rect);
                }
            }
            for plate in &current_row {
                rect_builder
                    .entry(plate.clone())
                    .and_modify(|r| r.max.y += 1.0)
                    .or_insert(Rect {
                        min: Vec2::new(plate.left as f32, y as f32),
                        max: Vec2::new(plate.right as f32 + 1.0, y as f32 + 1.0),
                    });
            }
            prev_row = current_row;
        }

        commands.entity(level_entity).with_children(|level| {
            for rect in wall_rects {
                let pixel_rect = Rect {
                    min: rect.min * grid_size as f32,
                    max: rect.max * grid_size as f32,
                };
                level.spawn((CollisionRect(pixel_rect), Transform::default()));
            }
        });
    }
}
