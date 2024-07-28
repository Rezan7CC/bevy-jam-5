use crate::player::PlayerStats;
use crate::{life_cycles, player, spawning, ui};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Paused,
    Running,
    Restarting,
    TimeOver,
    GameOver,
}

#[derive(Component)]
pub struct RemoveOnRestart;

pub fn system_restart_game(
    mut commands: Commands,
    query: Query<Entity, With<RemoveOnRestart>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_threats: ResMut<spawning::CurrentThreats>,
    mut player_stats: ResMut<player::PlayerStats>,
) {
    current_threats.0 = 0;
    player_stats.score = 0;
    player_stats.ducks_born = 0;
    player_stats.is_simulating = false;

    game_state.set(GameState::Running);
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn system_reset_remaining_time(mut player_stats: ResMut<player::PlayerStats>) {
    if player_stats.is_simulating {
        return;
    }
    player_stats.remaining_time = 300.0;
}

pub fn system_update_remaining_time(
    time: Res<Time>,
    mut player_stats: ResMut<player::PlayerStats>,
    mut game_state: ResMut<NextState<GameState>>,
    mut remaining_time_widget: Query<(&mut Text, &ui::GameStatusWidgets)>,
) {
    if !player_stats.is_simulating {
        player_stats.remaining_time -= time.delta_seconds();
        player_stats.remaining_time = player_stats.remaining_time.max(0.0);

        for (mut text, widget) in remaining_time_widget.iter_mut() {
            if *widget != ui::GameStatusWidgets::RemainingTime {
                continue;
            }
            text.sections[0].value = format!("Remaining Time: {:.0}", player_stats.remaining_time);
        }

        if player_stats.remaining_time <= 0.0 {
            game_state.set(GameState::TimeOver);
        }
    } else {
        for (mut text, widget) in remaining_time_widget.iter_mut() {
            if *widget != ui::GameStatusWidgets::RemainingTime {
                continue;
            }
            text.sections[0].value = "Remaining Time: Endless".to_string();
        }
    }
}

pub fn system_check_game_over_condition(
    player_stats: Res<player::PlayerStats>,
    mut game_state: ResMut<NextState<GameState>>,
    ducks_and_eggs_query: Query<
        Entity,
        Or<(
            With<life_cycles::Duckling>,
            With<life_cycles::Juvenile>,
            With<life_cycles::Adult>,
            With<life_cycles::Egg>,
        )>,
    >,
) {
    if player_stats.is_simulating {
        return;
    }

    if ducks_and_eggs_query.iter().count() == 1 {
        game_state.set(GameState::GameOver);
    }
}

pub fn system_update_game_status_ui(
    player_stats: Res<PlayerStats>,
    mut game_status_widgets: Query<(&mut Text, &ui::GameStatusWidgets)>,
    ducks_query: Query<
        Entity,
        Or<(
            With<life_cycles::Duckling>,
            With<life_cycles::Juvenile>,
            With<life_cycles::Adult>,
        )>,
    >,
) {
    for (mut text, widget) in game_status_widgets.iter_mut() {
        match *widget {
            ui::GameStatusWidgets::DucklingsBorn => {
                text.sections[0].value = format!("Ducks Born: {0}", player_stats.ducks_born);
            }
            ui::GameStatusWidgets::DucksAlive => {
                text.sections[0].value = format!("Ducks Alive: {0}", ducks_query.iter().count());
            }
            _ => {}
        }
    }
}

pub fn system_change_state_to_paused(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}
