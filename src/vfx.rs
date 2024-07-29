use bevy::prelude::*;
//use bevy::window::PrimaryWindow;
use crate::spawning;
use bevy_particle_systems::{
    ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing, VelocityModifier,
};

/*
pub fn spawn_particle_systems(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    loaded_assets: Res<spawning::LoadedAssets>,
) {
    let (camera, camera_transform) = camera.single();

    if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {

    }
}
*/

pub fn spawn_duck_eaten_effect(
    commands: &mut Commands,
    loaded_assets: &Res<spawning::LoadedAssets>,
    world_position: Vec2,
    start_color: Color,
) {
    let end_color = start_color.with_alpha(0.0);
    let position = Transform::from_translation(world_position.extend(0.0));
    
    commands.spawn((
        ParticleSystemBundle {
            transform: position,
            global_transform: GlobalTransform::from_translation(position.translation),
            particle_system: ParticleSystem {
                texture: loaded_assets.feather_image.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                max_particles: 25,
                initial_speed: (0.0..200.0).into(),
                initial_rotation: (0.0..360.0_f32.to_radians()).into(),
                rotation_speed: (-10.0..10.0).into(),
                lifetime: (0.3..0.5).into(),
                scale: 0.1.into(),
                velocity_modifiers: vec![
                    VelocityModifier::Drag(0.001.into()),
                    VelocityModifier::Vector(Vec3::new(0.0, -100.0, 0.0).into()),
                ],
                color: (start_color..end_color).into(),
                bursts: vec![ParticleBurst {
                    time: 0.0,
                    count: 25,
                }],
                ..ParticleSystem::oneshot()
            },
            ..default()
        },
        Playing,
    ));
}

pub fn spawn_duck_cycle_transition_effect(
    commands: &mut Commands,
    loaded_assets: &Res<spawning::LoadedAssets>,
    world_position: Vec2,
) {
    let start_color = Color::srgba(0.8, 0.8, 0.8, 0.5);
    let end_color = start_color.with_alpha(0.0);
    let position = Transform::from_translation(world_position.extend(5.0));
    
    commands.spawn((
        ParticleSystemBundle {
            transform: position,
            global_transform: GlobalTransform::from_translation(position.translation),
            particle_system: ParticleSystem {
                texture: loaded_assets.circle_image.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                max_particles: 25,
                initial_speed: (10.0..70.0).into(),
                initial_rotation: (0.0..360.0_f32.to_radians()).into(),
                lifetime: (0.3..0.4).into(),
                scale: (1.0..0.6).into(),
                velocity_modifiers: vec![
                    VelocityModifier::Drag(0.001.into()),
                    VelocityModifier::Vector(Vec3::new(0.0, -50.0, 0.0).into()),
                ],
                color: (start_color..end_color).into(),
                bursts: vec![ParticleBurst {
                    time: 0.0,
                    count: 25,
                }],
                ..ParticleSystem::oneshot()
            },
            ..default()
        },
        Playing,
    ));
}

pub fn spawn_food_eaten_effect(
    commands: &mut Commands,
    loaded_assets: &Res<spawning::LoadedAssets>,
    world_position: Vec2,
) {
    let start_color: Color = Color::srgba(0.65, 0.5, 0.4, 1.0);
    let end_color = start_color.with_alpha(0.0);
    let position = Transform::from_translation(world_position.extend(0.0));
    
    commands.spawn((
        ParticleSystemBundle {
            transform: position,
            global_transform: GlobalTransform::from_translation(position.translation),
            particle_system: ParticleSystem {
                texture: loaded_assets.circle_image.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                max_particles: 20,
                initial_speed: (10.0..250.0).into(),
                initial_rotation: (0.0..360.0_f32.to_radians()).into(),
                lifetime: (0.2..0.3).into(),
                scale: (0.25..0.15).into(),
                velocity_modifiers: vec![
                    VelocityModifier::Drag(0.001.into()),
                    VelocityModifier::Vector(Vec3::new(0.0, -450.0, 0.0).into()),
                ],
                color: (start_color..end_color).into(),
                bursts: vec![ParticleBurst {
                    time: 0.0,
                    count: 20,
                }],
                ..ParticleSystem::oneshot()
            },
            ..default()
        },
        Playing,
    ));
}

pub fn spawn_egg_hatched_effect(
    commands: &mut Commands,
    loaded_assets: &Res<spawning::LoadedAssets>,
    world_position: Vec2,
) {
    let start_color: Color = Color::srgba(0.8, 0.6, 0.5, 1.0);
    let end_color = start_color.with_alpha(0.0);
    let position = Transform::from_translation(world_position.extend(0.0));

    commands.spawn((
        ParticleSystemBundle {
            transform: position,
            global_transform: GlobalTransform::from_translation(position.translation),
            particle_system: ParticleSystem {
                texture: loaded_assets.egg_shell_image.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                max_particles: 10,
                initial_speed: (10.0..150.0).into(),
                initial_rotation: 180.0_f32.to_radians().into(),
                rotation_speed: (-10.0..10.0).into(),
                lifetime: (0.2..0.3).into(),
                scale: (0.5..0.3).into(),
                velocity_modifiers: vec![
                    VelocityModifier::Drag(0.001.into()),
                    VelocityModifier::Vector(Vec3::new(0.0, -550.0, 0.0).into()),
                ],
                color: (start_color..end_color).into(),
                bursts: vec![ParticleBurst {
                    time: 0.0,
                    count: 10,
                }],
                ..ParticleSystem::oneshot()
            },
            ..default()
        },
        Playing,
    ));
}
