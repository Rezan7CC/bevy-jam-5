use crate::{game_state, leaderboard, player, spawning};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Paused,
    Running,
    Restarting,
}

#[derive(Component)]
pub struct RemoveOnRestart;

pub fn system_restart_game(
    mut commands: Commands,
    query: Query<Entity, With<RemoveOnRestart>>,
    mut game_state: ResMut<NextState<game_state::GameState>>,
    mut current_threats: ResMut<spawning::CurrentThreats>,
    mut player_stats: ResMut<player::PlayerStats>,
    mut processed_leaderboard: ResMut<leaderboard::ProcessedLeaderboard>,
) {
    current_threats.0 = 0;
    player_stats.score = 0;
    player_stats.is_simulating = false;
    processed_leaderboard.last_player_score = 0;

    game_state.set(GameState::Running);
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
