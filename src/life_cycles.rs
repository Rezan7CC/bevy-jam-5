use crate::boid::Boid;
use crate::duck_boid::CloseAdults;
use crate::{duck_boid, movement, spawning};
use bevy::prelude::*;

#[derive(Component)]
pub struct Egg;
#[derive(Component)]
pub struct Duckling;
#[derive(Component)]
pub struct Juvenile;
#[derive(Component)]
pub struct Adult;

#[derive(Component, Default)]
pub struct LifeCycleTime(pub f32);

const TIME_FACTOR: f32 = 1.0;
pub const EGG_HATCH_TIME_MIN: f32 = 10.0 * TIME_FACTOR;
pub const EGG_HATCH_TIME_MAX: f32 = 30.0 * TIME_FACTOR;

pub const DUCKLING_TO_JUVENILE_TIME_MIN: f32 = 10.0 * TIME_FACTOR;
pub const DUCKLING_TO_JUVENILE_TIME_MAX: f32 = 30.0 * TIME_FACTOR;

pub const JUVENILE_TO_ADULT_TIME_MIN: f32 = 10.0 * TIME_FACTOR;
pub const JUVENILE_TO_ADULT_TIME_MAX: f32 = 30.0 * TIME_FACTOR;

pub fn system_decrease_lifecycle_time(time: Res<Time>, mut query: Query<&mut LifeCycleTime>) {
    for mut life_cycle_time in query.iter_mut() {
        life_cycle_time.0 -= time.delta_seconds();
    }
}

fn transition_life_cycle(
    next_cycle_time_min: f32,
    next_cycle_time_max: f32,
    next_cycle_sprite_name: &'static str,
    z_value: f32,
    asset_server: &Res<AssetServer>,
    entity_image: &mut Handle<Image>,
    life_cycle_time: &mut LifeCycleTime,
    transform: &mut Transform,
) -> bool {
    if life_cycle_time.0 > 0.0 {
        return false;
    }
    spawning::switch_sprite(next_cycle_sprite_name, asset_server, entity_image);
    transform.translation.z = z_value;
    life_cycle_time.0 =
        rand::random::<f32>() * (next_cycle_time_max - next_cycle_time_min) + next_cycle_time_min;
    true
}

pub fn system_hatch_eggs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &mut LifeCycleTime, &mut Handle<Image>), With<Egg>>,
) {
    for (entity, mut transform, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            "duckling.png",
            0.0,
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Egg>();
            commands.entity(entity).insert(Duckling);
            commands.entity(entity).insert(Boid);

            let random_direction =
                Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
            let velocity = random_direction * movement::MIN_VELOCITY;
            commands.entity(entity).insert(movement::Velocity(velocity));
        }
    }
}

pub fn system_duckling_to_juvenile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &mut LifeCycleTime, &mut Handle<Image>), With<Duckling>>,
) {
    for (entity, mut transform, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            JUVENILE_TO_ADULT_TIME_MIN,
            JUVENILE_TO_ADULT_TIME_MAX,
            "juvenile.png",
            1.0,
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Duckling>();
            commands.entity(entity).insert(Juvenile);
        }
    }
}

pub fn system_juvenile_to_adult(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &mut LifeCycleTime, &mut Handle<Image>), With<Juvenile>>,
) {
    for (entity, mut transform, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            "adult.png",
            2.0,
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Juvenile>();
            commands.entity(entity).insert(Adult);
            commands
                .entity(entity)
                .insert(duck_boid::CloseAdults::default());
            commands.entity(entity).insert(BreedingProgress::default());
        }
    }
}

#[derive(Component, Default)]
pub struct BreedingProgress(pub f32);

pub const BREEDING_DURATION: f32 = 10.0 * TIME_FACTOR;

pub fn system_breeding(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut BreedingProgress, &CloseAdults), With<Adult>>,
) {
    for (transform, mut breeding_progress, close_adults) in query.iter_mut() {
        if close_adults.0.is_empty() {
            continue;
        }
        breeding_progress.0 += time.delta_seconds();

        if breeding_progress.0 >= BREEDING_DURATION {
            spawning::spawn_boid(transform.translation.xy(), &mut commands, &asset_server);

            breeding_progress.0 = 0.0;
        }
    }
}
