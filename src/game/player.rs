use bevy::{
    app::{App, Plugin},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        schedule::{InternedScheduleLabel, ScheduleLabel},
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin {
    spawn_schedule: InternedScheduleLabel,
    input_schedule: InternedScheduleLabel,
}

impl PlayerPlugin {
    pub fn new(spawn_schedule: impl ScheduleLabel, input_schedule: impl ScheduleLabel) -> Self {
        Self {
            spawn_schedule: spawn_schedule.intern(),
            input_schedule: input_schedule.intern(),
        }
    }
}

const PLAYER_SPEED: f32 = 200.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn_schedule, spawn_player)
            .add_systems(self.input_schedule, move_player);
    }
}

fn spawn_player(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image_handle = images.add(Image::default());

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            image: image_handle,
            ..Default::default()
        },
        Transform::from_xyz(320.0, 320.0, 1.0),
        Player,
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Some(mut transform) = query.iter_mut().next() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    direction = direction.normalize_or_zero();
    transform.translation += (direction * PLAYER_SPEED * time.delta_secs()).extend(0.0);
}
