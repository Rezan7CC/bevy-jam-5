use crate::boid::Boid;
use crate::{movement, spawning};
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

pub const EGG_HATCH_TIME_MIN: f32 = 10.0;
pub const EGG_HATCH_TIME_MAX: f32 = 30.0;

pub const DUCKLING_TO_JUVENILE_TIME_MIN: f32 = 10.0;
pub const DUCKLING_TO_JUVENILE_TIME_MAX: f32 = 30.0;

pub const JUVENILE_TO_ADULT_TIME_MIN: f32 = 10.0;
pub const JUVENILE_TO_ADULT_TIME_MAX: f32 = 30.0;

pub fn system_decrease_lifecycle_time(time: Res<Time>, mut query: Query<&mut LifeCycleTime>) {
    for mut life_cycle_time in query.iter_mut() {
        life_cycle_time.0 -= time.delta_seconds();
    }
}

fn transition_life_cycle(
    next_cycle_time_min: f32,
    next_cycle_time_max: f32,
    next_cycle_sprite_name: &'static str,
    asset_server: &Res<AssetServer>,
    entity_image: &mut Handle<Image>,
    life_cycle_time: &mut LifeCycleTime,
) -> bool {
    if life_cycle_time.0 > 0.0 {
        return false;
    }
    spawning::switch_sprite(next_cycle_sprite_name, asset_server, entity_image);
    life_cycle_time.0 =
        rand::random::<f32>() * (next_cycle_time_max - next_cycle_time_min) + next_cycle_time_min;
    true
}

pub fn system_hatch_eggs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut LifeCycleTime, &mut Handle<Image>), With<Egg>>,
) {
    for (entity, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            "duckling.png",
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
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
    mut query: Query<(Entity, &mut LifeCycleTime, &mut Handle<Image>), With<Duckling>>,
) {
    for (entity, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            JUVENILE_TO_ADULT_TIME_MIN,
            JUVENILE_TO_ADULT_TIME_MAX,
            "juvenile.png",
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
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
    mut query: Query<(Entity, &mut LifeCycleTime, &mut Handle<Image>), With<Juvenile>>,
) {
    for (entity, mut life_cycle_time, mut entity_image) in query.iter_mut() {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            "adult.png",
            &asset_server,
            &mut entity_image,
            &mut life_cycle_time,
        );

        if transitioned {
            commands.entity(entity).remove::<Juvenile>();
            commands.entity(entity).insert(Adult);
        }
    }
}
