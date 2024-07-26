use crate::boid::Boid;
use crate::{duck_boid, movement, player, spawning, sprite_animation};
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

pub const TIME_FACTOR: f32 = 1.0;
pub const EGG_HATCH_TIME_MIN: f32 = 5.0 * TIME_FACTOR;
pub const EGG_HATCH_TIME_MAX: f32 = 15.0 * TIME_FACTOR;

pub const DUCKLING_TO_JUVENILE_TIME_MIN: f32 = 10.0 * TIME_FACTOR;
pub const DUCKLING_TO_JUVENILE_TIME_MAX: f32 = 30.0 * TIME_FACTOR;

pub const JUVENILE_TO_ADULT_TIME_MIN: f32 = 8.0 * TIME_FACTOR;
pub const JUVENILE_TO_ADULT_TIME_MAX: f32 = 15.0 * TIME_FACTOR;

pub fn system_decrease_lifecycle_time(time: Res<Time>, mut query: Query<&mut LifeCycleTime>) {
    for mut life_cycle_time in query.iter_mut() {
        life_cycle_time.0 -= time.delta_seconds();
    }
}

fn transition_life_cycle(
    next_cycle_time_min: f32,
    next_cycle_time_max: f32,
    z_value: f32,
    scale: f32,
    new_image: Handle<Image>,
    new_texture_atlas: Handle<TextureAtlasLayout>,
    new_animation_indices: sprite_animation::AnimationIndices,
    new_animation_timer: sprite_animation::AnimationTimer,
    entity_image: &mut Handle<Image>,
    texture_atlas: &mut TextureAtlas,
    animation_indices: &mut sprite_animation::AnimationIndices,
    animation_timer: &mut sprite_animation::AnimationTimer,
    life_cycle_time: &mut LifeCycleTime,
    transform: &mut Transform,
) -> bool {
    if life_cycle_time.0 > 0.0 {
        return false;
    }

    *animation_indices = new_animation_indices;
    *animation_timer = new_animation_timer;
    *entity_image = new_image;
    texture_atlas.layout = new_texture_atlas;
    texture_atlas.index = animation_indices.first;

    transform.translation.z = z_value;
    transform.scale = Vec3::splat(scale);
    life_cycle_time.0 =
        rand::random::<f32>() * (next_cycle_time_max - next_cycle_time_min) + next_cycle_time_min;
    true
}

pub fn system_hatch_eggs(
    mut commands: Commands,
    loaded_assets: Res<spawning::LoadedAssets>,
    mut player_stats: ResMut<player::PlayerStats>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut LifeCycleTime,
            &mut Handle<Image>,
            &mut TextureAtlas,
            &mut sprite_animation::AnimationIndices,
            &mut sprite_animation::AnimationTimer,
        ),
        With<Egg>,
    >,
) {
    for (
        entity,
        mut transform,
        mut life_cycle_time,
        mut entity_image,
        mut texture_atlas,
        mut animation_indices,
        mut animation_timer,
    ) in query.iter_mut()
    {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            0.0,
            1.25,
            loaded_assets.duckling_sprite.clone(),
            loaded_assets.duckling_atlas.clone(),
            sprite_animation::AnimationIndices {
                first: 0,
                last: 3,
                paused: false,
            },
            sprite_animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            &mut entity_image,
            &mut texture_atlas,
            &mut animation_indices,
            &mut animation_timer,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Egg>();
            commands.entity(entity).try_insert(Duckling);
            commands.entity(entity).try_insert(Boid);

            if !player_stats.is_simulating {
                player_stats.score += 1;
            }

            let random_direction =
                Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();

            let velocity_limits = movement::VelocityLimits::default();
            let velocity = random_direction * velocity_limits.min;
            commands
                .entity(entity)
                .try_insert(movement::Velocity(velocity))
                .try_insert(velocity_limits);
        }
    }
}

pub fn system_duckling_to_juvenile(
    mut commands: Commands,
    loaded_assets: Res<spawning::LoadedAssets>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut LifeCycleTime,
            &mut Handle<Image>,
            &mut TextureAtlas,
            &mut sprite_animation::AnimationIndices,
            &mut sprite_animation::AnimationTimer,
        ),
        With<Duckling>,
    >,
) {
    for (
        entity,
        mut transform,
        mut life_cycle_time,
        mut entity_image,
        mut texture_atlas,
        mut animation_indices,
        mut animation_timer,
    ) in query.iter_mut()
    {
        let transitioned = transition_life_cycle(
            JUVENILE_TO_ADULT_TIME_MIN,
            JUVENILE_TO_ADULT_TIME_MAX,
            1.0,
            1.5,
            loaded_assets.juvenile_sprite.clone(),
            loaded_assets.juvenile_atlas.clone(),
            sprite_animation::AnimationIndices {
                first: 0,
                last: 3,
                paused: false,
            },
            sprite_animation::AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            &mut entity_image,
            &mut texture_atlas,
            &mut animation_indices,
            &mut animation_timer,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Duckling>();
            commands.entity(entity).try_insert(Juvenile);
        }
    }
}

pub fn system_juvenile_to_adult(
    mut commands: Commands,
    loaded_assets: Res<spawning::LoadedAssets>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut LifeCycleTime,
            &mut Handle<Image>,
            &mut TextureAtlas,
            &mut sprite_animation::AnimationIndices,
            &mut sprite_animation::AnimationTimer,
        ),
        With<Juvenile>,
    >,
) {
    for (
        entity,
        mut transform,
        mut life_cycle_time,
        mut entity_image,
        mut texture_atlas,
        mut animation_indices,
        mut animation_timer,
    ) in query.iter_mut()
    {
        let transitioned = transition_life_cycle(
            DUCKLING_TO_JUVENILE_TIME_MIN,
            DUCKLING_TO_JUVENILE_TIME_MAX,
            2.0,
            2.0,
            loaded_assets.adult_sprite.clone(),
            loaded_assets.adult_atlas.clone(),
            sprite_animation::AnimationIndices {
                first: 0,
                last: 3,
                paused: false,
            },
            sprite_animation::AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            &mut entity_image,
            &mut texture_atlas,
            &mut animation_indices,
            &mut animation_timer,
            &mut life_cycle_time,
            &mut transform,
        );

        if transitioned {
            commands.entity(entity).remove::<Juvenile>();
            commands.entity(entity).try_insert(Adult);
            commands
                .entity(entity)
                .try_insert(duck_boid::CloseAdults::default());
        }
    }
}
