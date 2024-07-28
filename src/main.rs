// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod audio;
mod boid;
mod breeding;
mod cursor;
mod duck_boid;
mod food;
mod game_state;
mod leaderboard;
mod life_cycles;
mod movement;
mod player;
mod spawning;
mod sprite_animation;
mod threat_boid;
mod ui;
mod vfx;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_jornet::JornetPlugin;
use bevy::winit::WinitWindows;
use winit::window::Icon;
//use bevy::input::common_conditions::input_just_pressed;

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
                        fit_canvas_to_parent: true,
                        title: "Ducky Boids".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(JornetPlugin::with_leaderboard(
            env!("JORNET_LEADERBOARD_ID"),
            env!("JORNET_LEADERBOARD_KEY"),
            //&uuid::Uuid::NAMESPACE_X500.to_string(),
            //&uuid::Uuid::NAMESPACE_X500.to_string(),
        ))
        .add_plugins(bevy_particle_systems::ParticleSystemPlugin)
        .insert_resource(food::FoodPlacementTimer(Timer::from_seconds(
            food::FOOD_PLACEMENT_COOLDOWN,
            TimerMode::Once,
        )))
        .insert_resource(spawning::LoadedAssets::default())
        .insert_resource(spawning::CurrentThreats::default())
        .insert_resource(audio::ActiveAudioSources::default())
        .insert_resource(leaderboard::ProcessedLeaderboard::default())
        .insert_resource(player::PlayerStats::default())
        .insert_state(game_state::GameState::Paused)
        .add_systems(
            Startup,
            (
                setup,
                system_set_window_icon,
                leaderboard::system_setup_leaderboard,
                spawning::load_assets,
                ui::system_create_main_menu.after(spawning::load_assets),
                ui::system_spawn_leaderboard_ui.after(spawning::load_assets),
                spawning::system_spawn_boids.after(spawning::load_assets),
                spawning::system_spawn_threats.after(spawning::load_assets),
            ),
        )
        //.add_systems(
        //    OnEnter(game_state::GameState::Running),
        //    leaderboard::system_add_test_score,
        //)
        .add_systems(
            OnEnter(game_state::GameState::Restarting),
            game_state::system_restart_game,
        )
        .add_systems(
            OnExit(game_state::GameState::Restarting),
            (spawning::system_spawn_boids, spawning::system_spawn_threats),
        )
        .add_systems(
            OnEnter(game_state::GameState::TimeOver),
            (
                ui::system_create_time_over_menu,
                game_state::system_change_state_to_paused,
            ),
        )
        .add_systems(
            OnEnter(game_state::GameState::GameOver),
            (
                ui::system_create_game_over_menu,
                game_state::system_change_state_to_paused,
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
            )
                .run_if(in_state(game_state::GameState::Running)),
        )
        .add_systems(
            Update,
            (
                life_cycles::system_decrease_lifecycle_time,
                life_cycles::system_hatch_eggs,
                life_cycles::system_duckling_to_juvenile,
                life_cycles::system_juvenile_to_adult,
                cursor::system_update_game_cursor_position,
                cursor::system_update_game_cursor_image,
                game_state::system_update_remaining_time,
                game_state::system_update_game_status_ui,
                game_state::system_check_game_over_condition,
            )
                .run_if(in_state(game_state::GameState::Running)),
        )
        .add_systems(
            Update,
            (
                ui::system_ui_actions,
                ui::system_button_color,
                leaderboard::system_display_leaderboard,
                audio::system_update_active_audio_sources,
                //vfx::spawn_particle_systems.run_if(input_just_pressed(MouseButton::Left)),
            ),
        )
        .add_systems(
            OnEnter(game_state::GameState::Running),
            (
                audio::system_start_soundtrack,
                game_state::system_reset_remaining_time,
                cursor::system_enable_game_cursor,
            ),
        )
        .add_systems(
            OnEnter(game_state::GameState::Paused),
            (
                audio::system_stop_soundtrack,
                cursor::system_disable_game_cursor,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn system_set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/duck_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}