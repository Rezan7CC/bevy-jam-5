use crate::spawning::LoadedAssets;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

#[derive(Component)]
pub struct Soundtrack;

#[derive(Component)]
pub struct AudioSource;

#[derive(Resource, Default)]
pub struct ActiveAudioSources(pub i32);

pub fn system_start_soundtrack(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Soundtrack_edited.mp3"),
            settings: PlaybackSettings {
                volume: Volume::new(0.16),
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

pub fn system_update_active_audio_sources(
    query: Query<Entity, With<AudioSource>>,
    mut active_audio_sources: ResMut<ActiveAudioSources>,
) {
    active_audio_sources.0 = query.iter().count() as i32;
}

pub fn play_egg_pop(
    loaded_assets: &Res<LoadedAssets>,
    commands: &mut Commands,
    active_audio_sources: &Res<ActiveAudioSources>,
) {
    if active_audio_sources.0 >= 50 {
        return;
    }
    let volume = if active_audio_sources.0 > 20 {
        0.07
    } else {
        0.25
    };

    commands.spawn((
        AudioBundle {
            source: loaded_assets.pop_sound.clone(),
            settings: PlaybackSettings {
                volume: Volume::new(volume),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        },
        AudioSource,
    ));
}

pub fn play_duck_eaten(
    loaded_assets: &Res<LoadedAssets>,
    commands: &mut Commands,
    active_audio_sources: &Res<ActiveAudioSources>,
) {
    if active_audio_sources.0 >= 50 {
        return;
    }
    let volume = if active_audio_sources.0 > 20 {
        0.1
    } else {
        0.2
    };

    commands.spawn((
        AudioBundle {
            source: loaded_assets.duck_eaten_sound.clone(),
            settings: PlaybackSettings {
                volume: Volume::new(volume),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        },
        AudioSource,
    ));
}

pub fn play_button_clicked(loaded_assets: &Res<LoadedAssets>, commands: &mut Commands) {
    commands.spawn(AudioBundle {
        source: loaded_assets.button_clicked_sound.clone(),
        settings: PlaybackSettings {
            volume: Volume::new(0.2),
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}
