use crate::spawning::LoadedAssets;
use crate::{audio, boid, movement, sprite_animation};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Default)]
pub struct Threat {
    pub eating_cooldown: f32,
    pub running: bool,
}

const THREAT_VISIBILITY_RADIUS_2: f32 = 145.0 * 145.0 * 100.0;
const TOWARDS_CLOSEST_DUCK_FACTOR: f32 = 300.0;
const DECELERATION_FACTOR: f32 = 150.0;
const THREAT_EATING_RADIUS_2: f32 = 30.0 * 30.0;
const THREAT_EATING_COOLDOWN_DURATION: f32 = 2.0;
pub fn system_boid_towards_closest_duck(
    time: Res<Time>,
    mut commands: Commands,
    duck_query: Query<(Entity, &Transform), (With<boid::Boid>, Without<Threat>)>,
    mut threat_query: Query<(&Transform, &mut movement::Velocity, &mut Threat)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    loaded_assets: Res<LoadedAssets>,
    active_audio_sources: Res<audio::ActiveAudioSources>,
) {
    for (threat_transform, mut threat_velocity, mut threat) in threat_query.iter_mut() {
        threat.eating_cooldown -= time.delta_seconds();

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

        if closest_duck.is_some() && threat.eating_cooldown <= 0.0 {
            if closest_duck.unwrap().1 <= THREAT_EATING_RADIUS_2 {
                if let Some(mut entity_cmd) = commands.get_entity(closest_duck.unwrap().2) {
                    entity_cmd.despawn();
                    audio::play_duck_eaten(&loaded_assets, &mut commands, &active_audio_sources);
                }
                threat.eating_cooldown = THREAT_EATING_COOLDOWN_DURATION;
                continue;
            }

            let direction =
                (closest_duck.unwrap().0 - threat_transform.translation.xy()).normalize();
            threat_velocity.0 += direction * TOWARDS_CLOSEST_DUCK_FACTOR * time.delta_seconds();
        } else {
            let window = if let Ok(window) = window_query.get_single() {
                window
            } else {
                continue;
            };

            if movement::is_avoiding_edge(threat_transform.translation.xy(), window) {
                continue;
            }

            let length = threat_velocity.0.length();
            if length > 0.0 {
                let new_length = (length - DECELERATION_FACTOR * time.delta_seconds()).max(0.0);
                threat_velocity.0 = threat_velocity.0 / length * new_length;
            }
        }
    }
}

pub fn system_update_threat_animation(
    mut query: Query<(
        &mut TextureAtlas,
        &movement::Velocity,
        &mut sprite_animation::AnimationIndices,
        &mut sprite_animation::AnimationTimer,
        &mut Threat,
    )>,
    loaded_assets: Res<LoadedAssets>,
) {
    for (mut texture_atlas, velocity, mut animation_indices, mut animation_timer, mut threat) in
        query.iter_mut()
    {
        let speed = velocity.0.length();

        if speed > 100.0 {
            if !threat.running {
                threat.running = true;
                texture_atlas.layout = loaded_assets.threat_running_atlas.clone();
                texture_atlas.index = 0;

                *animation_indices = sprite_animation::AnimationIndices {
                    first: 0,
                    last: 7,
                    paused: false,
                };
                *animation_timer = sprite_animation::AnimationTimer(Timer::from_seconds(
                    0.15,
                    TimerMode::Repeating,
                ));
            }
        } else if threat.running {
            threat.running = false;
            texture_atlas.layout = loaded_assets.threat_walking_atlas.clone();
            texture_atlas.index = 0;

            *animation_indices = sprite_animation::AnimationIndices {
                first: 0,
                last: 3,
                paused: false,
            };
            *animation_timer =
                sprite_animation::AnimationTimer(Timer::from_seconds(1.5, TimerMode::Repeating));
        }
    }
}
