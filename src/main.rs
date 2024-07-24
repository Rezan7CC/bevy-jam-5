// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod boid;
mod breeding;
mod duck_boid;
mod food;
mod life_cycles;
mod movement;
mod spawning;
mod sprite_animation;
mod threat_boid;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Duck Boids".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(food::FoodPlacementTimer(Timer::from_seconds(
            food::FOOD_PLACEMENT_COOLDOWN,
            TimerMode::Once,
        )))
        .insert_resource(spawning::LoadedAssets::default())
        .insert_resource(spawning::CurrentThreats::default())
        .add_systems(
            Startup,
            (
                setup,
                spawning::load_assets,
                spawning::system_spawn_boids.after(spawning::load_assets),
                spawning::system_spawn_threats.after(spawning::load_assets),
            ),
        )
        .add_systems(
            Update,
            (
                spawning::system_continuous_threat_spawning,
                breeding::system_build_relationships,
                breeding::system_update_relationships
                    .after(breeding::system_build_relationships)
                    .after(movement::system_movement),
                breeding::system_breeding,
                boid::system_boid_separation,
                boid::system_boid_alignment_and_cohesion
                    .after(boid::system_boid_separation)
                    .before(movement::system_clamp_velocity),
                duck_boid::system_boid_update_close_adults,
                duck_boid::system_boids_food.before(movement::system_clamp_velocity),
                duck_boid::system_boid_mating_attraction
                    .after(duck_boid::system_boid_update_close_adults)
                    .before(movement::system_clamp_velocity),
                duck_boid::system_boids_ducklings_towards_adults
                    .before(movement::system_clamp_velocity),
                duck_boid::system_boids_avoid_threat.before(movement::system_clamp_velocity),
                threat_boid::system_boid_towards_closest_duck
                    .before(movement::system_clamp_velocity),
                threat_boid::system_update_threat_animation.after(movement::system_clamp_velocity),
                movement::system_clamp_velocity,
                movement::system_flip_based_on_velocity,
                movement::system_avoid_edges.after(movement::system_clamp_velocity),
                movement::system_movement.after(movement::system_avoid_edges),
                food::system_place_food_on_input,
                sprite_animation::system_animate_sprites.after(movement::system_movement),
            ),
        )
        .add_systems(
            Update,
            (
                life_cycles::system_decrease_lifecycle_time,
                life_cycles::system_hatch_eggs,
                life_cycles::system_duckling_to_juvenile,
                life_cycles::system_juvenile_to_adult,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
