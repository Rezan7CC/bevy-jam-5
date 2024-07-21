use crate::spawning;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const FOOD_PLACEMENT_COOLDOWN: f32 = 1.0;

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoodPlacementTimer(pub Timer);

pub fn system_place_food_on_input(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut food_placement_timer: ResMut<FoodPlacementTimer>,
) {
    let window = if let Ok(window) = window_query.get_single() {
        window
    } else {
        return;
    };

    let cursor_position = if let Some(position) = window.cursor_position() {
        position
    } else {
        return;
    };

    let (camera, camera_transform) =
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            (camera, camera_transform)
        } else {
            return;
        };

    if !food_placement_timer.0.tick(time.delta()).finished() {
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    food_placement_timer.0.reset();

    let world_position = if let Some(world_position) =
        camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        world_position
    } else {
        return;
    };

    spawning::spawn_food(world_position, &mut commands, &asset_server);
}
