use crate::movement::Velocity;
use bevy::prelude::*;
use bevy::utils::HashMap;

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

const VISIBILITY_RADIUS_2: f32 = 200.0 * 200.0;
const ALIGN_FACTOR: f32 = 0.1;
pub fn system_boid_alignment(
    mut queries: ParamSet<(
        Query<(Entity, &Velocity, &Transform), With<Boid>>,
        Query<(Entity, &mut Velocity), With<Boid>>,
    )>,
) {
    let mut velocity_map: HashMap<Entity, Vec2> = Default::default();

    let query = queries.p0();
    for (entity, _, transform) in query.iter() {
        let mut velocity_average: Vec2 = Vec2::ZERO;
        let mut neighbors: i32 = 0;

        for (other_entity, other_velocity, other_transform) in query.iter() {
            if entity == other_entity {
                continue;
            }
            if (transform.translation.xy() - other_transform.translation.xy()).length_squared()
                > VISIBILITY_RADIUS_2
            {
                continue;
            }

            velocity_average += other_velocity.0;
            neighbors += 1;
        }

        if neighbors == 0 {
            continue;
        }
        velocity_average /= neighbors as f32;
        velocity_map.insert(entity, velocity_average * ALIGN_FACTOR);
    }

    for (entity, mut velocity) in queries.p1().iter_mut() {
        if let Some(velocity_addition) = velocity_map.get(&entity) {
            velocity.0 += *velocity_addition;
        }
    }
}
