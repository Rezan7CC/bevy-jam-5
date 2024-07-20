use bevy::prelude::*;

pub const MIN_VELOCITY: f32 = 50.0;
pub const MAX_VELOCITY: f32 = 100.0;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn system_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

pub fn system_update_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = velocity.0.clamp_length(MIN_VELOCITY, MAX_VELOCITY);
    }
}
