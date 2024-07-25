use crate::{player, ui};
use bevy::prelude::*;
use bevy_jornet::{Leaderboard, Score};
use std::cmp::Ordering;

pub(crate) fn system_setup_leaderboard(mut leaderboard: ResMut<Leaderboard>) {
    // `None` will create a new user with a random name
    leaderboard.create_player(None);

    leaderboard.refresh_leaderboard();
}

pub fn system_add_test_score(leaderboard: ResMut<Leaderboard>) {
    if leaderboard.get_player().is_none() {
        return;
    }

    let random_score = rand::random::<f32>() * 100.0;
    leaderboard.send_score(random_score);
}

#[derive(Component)]
pub enum LeaderboardMarker {
    Number,
    Score,
    Player,
}

#[derive(Resource, Default)]
pub struct ProcessedLeaderboard {
    pub scores: Vec<Score>,
    pub last_player_score: i32,
}

pub fn system_display_leaderboard(
    leaderboard: Res<Leaderboard>,
    mut commands: Commands,
    root_ui: Query<(Entity, &LeaderboardMarker)>,
    mut processed_leaderboard: ResMut<ProcessedLeaderboard>,
    player_stats: Res<player::PlayerStats>,
) {
    let player = if let Some(player) = leaderboard.get_player() {
        player
    } else {
        return;
    };

    let score_changed = processed_leaderboard.last_player_score != player_stats.score;
    processed_leaderboard.last_player_score = player_stats.score;
    let player_score: f32 = player_stats.score as f32;

    let leaderboard_changed = leaderboard.is_changed();
    if leaderboard_changed {
        processed_leaderboard.scores = leaderboard.get_leaderboard();
        processed_leaderboard
            .scores
            .sort_unstable_by(|s1, s2| s2.score.partial_cmp(&s1.score).unwrap_or(Ordering::Equal));
    }

    let recreate_leaderboard =
        leaderboard_changed || (score_changed && !processed_leaderboard.scores.is_empty());

    if recreate_leaderboard {
        let player_index = processed_leaderboard
            .scores
            .binary_search_by(|s| {
                s.score
                    .partial_cmp(&player_score)
                    .unwrap_or(Ordering::Equal)
                    .reverse()
            })
            .unwrap_or_else(|i| i);

        for (root_entity, marker) in &root_ui {
            commands.entity(root_entity).despawn_descendants();

            if player_index < 10 {
                add_leaderboard_entry(
                    0,
                    9,
                    player_index,
                    &mut commands,
                    root_entity,
                    marker,
                    &processed_leaderboard.scores,
                    &player.name,
                    player_score,
                );
            } else {
                add_leaderboard_entry(
                    0,
                    2,
                    player_index,
                    &mut commands,
                    root_entity,
                    marker,
                    &processed_leaderboard.scores,
                    &player.name,
                    player_score,
                );

                commands.entity(root_entity).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "--",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::srgb(0.45, 0.45, 0.45),
                            ..default()
                        },
                    ));
                });

                add_leaderboard_entry(
                    (player_index - 2) as i32,
                    (player_index + 2) as i32,
                    player_index,
                    &mut commands,
                    root_entity,
                    marker,
                    &processed_leaderboard.scores,
                    &player.name,
                    player_score,
                );
            }
        }
    }
}

fn add_leaderboard_entry(
    start_index: i32,
    last_index: i32,
    player_index: usize,
    commands: &mut Commands,
    root_entity: Entity,
    marker: &LeaderboardMarker,
    leaderboard: &[Score],
    player_name: &String,
    player_score: f32,
) {
    let start_index = start_index.max(0) as usize;
    let last_index = last_index.min((leaderboard.len() as i32 - 1).max(0)) as usize;
    let mut added_player = false;

    for i in start_index..last_index + 1 {
        if i == player_index {
            added_player = true;
            commands.entity(root_entity).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    match marker {
                        LeaderboardMarker::Number => (i + 1).to_string() + ".",
                        LeaderboardMarker::Score => format!("{} ", player_score as i32),
                        LeaderboardMarker::Player => {
                            player_name
                                .to_owned()
                                .split(' ')
                                .next()
                                .unwrap()
                                .to_string()
                                + " (You)"
                        }
                    },
                    TextStyle {
                        font_size: match marker {
                            LeaderboardMarker::Number => 17.0,
                            LeaderboardMarker::Score => 17.0,
                            LeaderboardMarker::Player => 17.0,
                        },
                        color: ui::PRESSED_BUTTON,
                        ..default()
                    },
                ));
            });
        }

        if leaderboard.is_empty() || (i == last_index && player_index < last_index) {
            break;
        }

        let player_adjusted_index = if added_player { i + 1 } else { i };

        let score = &leaderboard[i];
        commands.entity(root_entity).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                match marker {
                    LeaderboardMarker::Number => (player_adjusted_index + 1).to_string() + ".",
                    LeaderboardMarker::Score => format!("{} ", score.score as i32),
                    LeaderboardMarker::Player => {
                        score.player.clone().split(' ').next().unwrap().to_string()
                    }
                },
                TextStyle {
                    font_size: match marker {
                        LeaderboardMarker::Number => 15.0,
                        LeaderboardMarker::Score => 15.0,
                        LeaderboardMarker::Player => 15.0,
                    },
                    color: ui::TEXT_COLOR,
                    ..default()
                },
            ));
        });
    }

    if leaderboard.is_empty()
        || player_index != leaderboard.len()
        || last_index != leaderboard.len() - 1
    {
        return;
    }
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            match marker {
                LeaderboardMarker::Number => (player_index + 1).to_string() + ".",
                LeaderboardMarker::Score => format!("{} ", player_score as i32),
                LeaderboardMarker::Player => {
                    player_name
                        .to_owned()
                        .split(' ')
                        .next()
                        .unwrap()
                        .to_string()
                        + " (You)"
                }
            },
            TextStyle {
                font_size: match marker {
                    LeaderboardMarker::Number => 17.0,
                    LeaderboardMarker::Score => 17.0,
                    LeaderboardMarker::Player => 17.0,
                },
                color: ui::PRESSED_BUTTON,
                ..default()
            },
        ));
    });
}
