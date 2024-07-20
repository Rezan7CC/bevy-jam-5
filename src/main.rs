// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod boid;
mod movement;
mod spawning;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, (setup, spawning::system_spawn_boids))
        .add_systems(
            Update,
            (
                boid::system_boid_separation,
                boid::system_boid_alignment_and_cohesion.after(boid::system_boid_separation),
                movement::system_update_velocity.after(boid::system_boid_alignment_and_cohesion),
                movement::system_avoid_edges.after(movement::system_update_velocity),
                movement::system_movement.after(movement::system_avoid_edges),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
