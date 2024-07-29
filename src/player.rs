use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerStats {
    pub score: i32,
    pub ducks_born: i32,
    pub is_simulating: bool,
    pub remaining_time: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            score: 0,
            ducks_born: 0,
            is_simulating: false,
            remaining_time: 240.0,
        }
    }
}
