use crate::food;
use crate::spawning::LoadedAssets;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameCursor;

const CURSOR_SIZE: f32 = 320.0;

pub fn system_enable_game_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    let mut window = if let Ok(window) = windows.get_single_mut() {
        window
    } else {
        return;
    };

    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: loaded_assets.cursor_empty_sprite.clone().into(),
            z_index: ZIndex::Global(15),
            transform: Transform {
                translation: cursor_spawn,
                scale: Vec3::splat(0.25),
                ..default()
            },
            style: Style {
                //display: Display::None,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        GameCursor,
    ));
}

pub fn system_disable_game_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    cursor_query: Query<Entity, With<GameCursor>>,
) {
    let mut window = if let Ok(window) = windows.get_single_mut() {
        window
    } else {
        return;
    };

    let cursor = if let Ok(cursor) = cursor_query.get_single() {
        cursor
    } else {
        return;
    };

    window.cursor.visible = true;
    commands.entity(cursor).despawn();
}

pub fn system_update_game_cursor_position(
    window: Query<&Window>,
    mut cursor: Query<&mut Style, With<GameCursor>>,
) {
    let window = if let Ok(window) = window.get_single() {
        window
    } else {
        return;
    };

    let mut cursor_style = if let Ok(cursor) = cursor.get_single_mut() {
        cursor
    } else {
        return;
    };

    if let Some(position) = window.cursor_position() {
        cursor_style.left = Val::Px(position.x - CURSOR_SIZE * 0.5 + 13.0);
        cursor_style.top = Val::Px(position.y - CURSOR_SIZE * 0.5 + 5.0);
    }
}

pub fn system_update_game_cursor_image(
    mut cursor: Query<&mut UiImage, With<GameCursor>>,
    loaded_assets: Res<LoadedAssets>,
    food_placement_timer: Res<food::FoodPlacementTimer>,
) {
    let mut cursor_image = if let Ok(cursor) = cursor.get_single_mut() {
        cursor
    } else {
        return;
    };

    if food_placement_timer.0.finished() {
        if cursor_image.texture != loaded_assets.cursor_food_sprite {
            cursor_image.texture = loaded_assets.cursor_food_sprite.clone();
        }
    } else if cursor_image.texture != loaded_assets.cursor_empty_sprite {
        cursor_image.texture = loaded_assets.cursor_empty_sprite.clone();
    }
}
