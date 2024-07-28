use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

#[derive(Component)]
pub struct Soundtrack;

pub fn system_start_soundtrack(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Soundtrack_edited.wav"),
            settings: PlaybackSettings {
                volume: Volume::new(0.07),
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        Soundtrack,
    ));
}

pub fn system_stop_soundtrack(mut commands: Commands, query: Query<Entity, With<Soundtrack>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
