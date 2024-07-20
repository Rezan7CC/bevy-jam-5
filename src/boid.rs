use crate::movement::Velocity;
use bevy::prelude::*;

#[derive(Component)]
pub struct Boid;

const PROTECTED_RADIUS_2: f32 = 100.0 * 100.0;
const AVOID_FACTOR: f32 = 0.1;
pub fn system_boid_separation(
    transform_query: Query<(Entity, &Transform), With<Boid>>,
    mut velocity_query: Query<(Entity, &mut Velocity), With<Boid>>,
) {
    for (entity, transform) in transform_query.iter() {
        let mut avoid_vector: Vec2 = Vec2::ZERO;

        for (other_entity, other_transform) in transform_query.iter() {
            if entity == other_entity {
                continue;
            }
            if (transform.translation.xy() - other_transform.translation.xy()).length_squared()
                > PROTECTED_RADIUS_2
            {
                continue;
            }

            avoid_vector += transform.translation.xy() - other_transform.translation.xy();
        }

        velocity_query.get_mut(entity).unwrap().1 .0 += avoid_vector * AVOID_FACTOR;
    }
}
