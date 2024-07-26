use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerStats {
    pub score: i32,
    pub is_simulating: bool,
}
