use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const MIN_VELOCITY: f32 = 50.0;
pub const MAX_VELOCITY: f32 = 100.0;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn system_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

pub fn system_clamp_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = velocity.0.clamp_length(MIN_VELOCITY, MAX_VELOCITY);
    }
}

const EDGE_MARGIN: f32 = 100.0;
const TURN_FACTOR: f32 = 200.0;
pub fn system_avoid_edges(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    for (mut velocity, transform) in query.iter_mut() {
        if transform.translation.x < -window_width * 0.5 + EDGE_MARGIN {
            velocity.0.x += TURN_FACTOR * time.delta_seconds();
        } else if transform.translation.x > window_width * 0.5 - EDGE_MARGIN {
            velocity.0.x -= TURN_FACTOR * time.delta_seconds();
        }

        if transform.translation.y < -window_height * 0.5 + EDGE_MARGIN {
            velocity.0.y += TURN_FACTOR * time.delta_seconds();
        } else if transform.translation.y > window_height * 0.5 - EDGE_MARGIN {
            velocity.0.y -= TURN_FACTOR * time.delta_seconds();
        }
    }
}
