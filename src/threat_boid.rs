use crate::{boid, movement};
use bevy::prelude::*;

#[derive(Component)]
pub struct Threat;

const THREAT_VISIBILITY_RADIUS_2: f32 = 120.0 * 150.0;
const TOWARDS_CLOSEST_DUCK_FACTOR: f32 = 300.0;
const DECELERATION_FACTOR: f32 = 150.0;
const THREAT_EATING_RADIUS_2: f32 = 20.0 * 20.0;
pub fn system_boid_towards_closest_duck(
    time: Res<Time>,
    mut commands: Commands,
    duck_query: Query<(Entity, &Transform), (With<boid::Boid>, Without<Threat>)>,
    mut threat_query: Query<(&Transform, &mut movement::Velocity), With<Threat>>,
) {
    for (threat_transform, mut threat_velocity) in threat_query.iter_mut() {
        let mut closest_duck: Option<(Vec2, f32, Entity)> = None;
        for (duck_entity, duck_transform) in duck_query.iter() {
            let distance_2 = (duck_transform.translation.xy() - threat_transform.translation.xy())
                .length_squared();
            if distance_2 < THREAT_VISIBILITY_RADIUS_2
                && (closest_duck.is_none() || distance_2 < closest_duck.unwrap().1)
            {
                closest_duck = Some((duck_transform.translation.xy(), distance_2, duck_entity));
            }
        }

        if closest_duck.is_some() {
            if closest_duck.unwrap().1 <= THREAT_EATING_RADIUS_2 {
                commands.entity(closest_duck.unwrap().2).despawn();
                continue;
            }

            let direction =
                (closest_duck.unwrap().0 - threat_transform.translation.xy()).normalize();
            threat_velocity.0 += direction * TOWARDS_CLOSEST_DUCK_FACTOR * time.delta_seconds();
        } else {
            let length = threat_velocity.0.length();
            if length > 0.0 {
                let new_length = (length - DECELERATION_FACTOR * time.delta_seconds()).max(0.0);
                threat_velocity.0 = threat_velocity.0 / length * new_length;
            }
        }
    }
}
