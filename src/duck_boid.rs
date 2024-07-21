use crate::boid::Boid;
use crate::movement::Velocity;
use crate::{boid, life_cycles};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Default)]
pub struct CloseAdults(pub Vec<(Vec2, Entity)>);

const CLOSE_ADULTS_RADIUS_2: f32 = 75.0 * 75.0;
pub fn system_boid_update_close_adults(
    mut queries: ParamSet<(
        Query<(Entity, &Transform), (With<Boid>, With<life_cycles::Adult>)>,
        Query<(Entity, &mut CloseAdults), (With<Boid>, With<life_cycles::Adult>)>,
    )>,
) {
    let mut close_adults_map: HashMap<Entity, Vec<(Vec2, Entity)>> = Default::default();

    let query = queries.p0();
    for (entity, transform) in query.iter() {
        let mut close_adults_array: Vec<(Vec2, Entity)> = Vec::with_capacity(10);

        for (other_entity, other_transform) in query.iter() {
            if entity == other_entity {
                continue;
            }
            if (transform.translation.xy() - other_transform.translation.xy()).length_squared()
                < CLOSE_ADULTS_RADIUS_2
            {
                close_adults_array.push((other_transform.translation.xy(), other_entity));
            }
        }
        close_adults_map.insert(entity, close_adults_array);
    }

    let mut mut_query = queries.p1();
    for (entity, mut close_adults) in mut_query.iter_mut() {
        close_adults
            .0
            .clone_from(close_adults_map.get(&entity).unwrap());
    }
}

const MATING_VISIBILITY_RADIUS_2: f32 = boid::VISIBILITY_RADIUS_2;
const MATING_MIN_DISTANCE_2: f32 = 20.0 * 20.0;
const TOWARDS_LONELY_ADULT_FACTOR: f32 = 400.0;
const AVOID_ADULT_GROUP_FACTOR: f32 = 400.0;

#[derive(Default)]
struct CloseAdultsMapEntry {
    biggest_adult_group: Option<(Vec2, i32)>, // (avg position, number of adults)
    closest_lonely_adult_position: Option<(Vec2, f32)>, // (position, distance squared)
}

pub fn system_boid_mating_attraction(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(Entity, &Transform, &CloseAdults), (With<Boid>, With<life_cycles::Adult>)>,
        Query<(Entity, &mut Velocity, &Transform), (With<Boid>, With<life_cycles::Adult>)>,
    )>,
) {
    let mut close_adults_map: HashMap<Entity, CloseAdultsMapEntry> = Default::default();

    let query = queries.p0();
    for (entity, transform, _) in query.iter() {
        let mut biggest_adult_group: Option<(Vec2, i32)> = None; // (avg position, number of adults)
        let mut closest_lonely_adult_position: Option<(Vec2, f32)> = None; // (position, distance squared)

        for (other_entity, other_transform, other_close_adults) in query.iter() {
            if entity == other_entity {
                continue;
            }
            if (transform.translation.xy() - other_transform.translation.xy()).length_squared()
                > MATING_VISIBILITY_RADIUS_2
            {
                continue;
            }

            if other_close_adults.0.is_empty()
                || (other_close_adults.0.len() == 1 && other_close_adults.0[0].1 == entity)
            {
                let distance_squared = (transform.translation.xy()
                    - other_transform.translation.xy())
                .length_squared();

                if closest_lonely_adult_position.is_none()
                    || distance_squared < closest_lonely_adult_position.unwrap().1
                {
                    closest_lonely_adult_position =
                        Some((other_transform.translation.xy(), distance_squared));
                }
            }

            if closest_lonely_adult_position.is_some() {
                continue;
            }

            let close_adults_numb = other_close_adults.0.len() as i32;
            if biggest_adult_group.is_none() || close_adults_numb > biggest_adult_group.unwrap().1 {
                let avg_position = other_close_adults
                    .0
                    .iter()
                    .fold(Vec2::ZERO, |acc, entry| acc + entry.0)
                    / close_adults_numb as f32;
                biggest_adult_group = Some((avg_position, close_adults_numb));
            }
        }

        close_adults_map.insert(
            entity,
            CloseAdultsMapEntry {
                biggest_adult_group,
                closest_lonely_adult_position,
            },
        );
    }

    let mut mut_query = queries.p1();
    for (entity, mut velocity, transform) in mut_query.iter_mut() {
        let entry = close_adults_map.get(&entity);
        if entry.is_none() {
            continue;
        }
        let entry = entry.unwrap();

        if entry.closest_lonely_adult_position.is_some() {
            let distance_2 = entry.closest_lonely_adult_position.unwrap().1;
            if distance_2 > MATING_MIN_DISTANCE_2 {
                let direction = (entry.closest_lonely_adult_position.unwrap().0
                    - transform.translation.xy())
                .normalize();
                velocity.0 += direction * TOWARDS_LONELY_ADULT_FACTOR * time.delta_seconds();
            }
        } else if entry.biggest_adult_group.is_some() {
            let direction =
                (entry.biggest_adult_group.unwrap().0 - transform.translation.xy()).normalize();
            velocity.0 -= direction * AVOID_ADULT_GROUP_FACTOR * time.delta_seconds();
        }
    }
}
