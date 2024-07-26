use crate::boid::Boid;
use crate::food::Food;
use crate::{life_cycles, movement, sprite_animation, threat_boid};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct LoadedAssets {
    pub cursor_empty_sprite: Handle<Image>,
    pub cursor_food_sprite: Handle<Image>,

    egg_sprite: Handle<Image>,

    pub duckling_sprite: Handle<Image>,
    pub duckling_atlas: Handle<TextureAtlasLayout>,

    pub juvenile_sprite: Handle<Image>,
    pub juvenile_atlas: Handle<TextureAtlasLayout>,

    pub adult_sprite: Handle<Image>,
    pub adult_atlas: Handle<TextureAtlasLayout>,

    tabby_sprite: Handle<Image>,
    threat_sprites: Vec<Handle<Image>>,
    pub threat_running_atlas: Handle<TextureAtlasLayout>,
    pub threat_walking_atlas: Handle<TextureAtlasLayout>,

    heart_sprite_sheet: Handle<Image>,
    heart_atlas_layout: Handle<TextureAtlasLayout>,

    food_sprites: Vec<Handle<Image>>,
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut loaded_assets: ResMut<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    loaded_assets.cursor_empty_sprite = asset_server.load("cursor/frame_0_delay-0.1s.png");
    loaded_assets.cursor_food_sprite = asset_server.load("cursor/frame_3_delay-0.1s.png");

    loaded_assets.egg_sprite = asset_server.load("egg.png");

    loaded_assets.duckling_sprite = asset_server.load("ducks/duckling_spritesheet.png");
    let duckling_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 1, None, None);
    loaded_assets.duckling_atlas = texture_atlas_layouts.add(duckling_layout);

    loaded_assets.juvenile_sprite = asset_server.load("ducks/juvenile_spritesheet.png");
    let juvenile_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 1, None, Some(UVec2::new(0, 33)));
    loaded_assets.juvenile_atlas = texture_atlas_layouts.add(juvenile_layout);

    loaded_assets.adult_sprite = asset_server.load("ducks/adult_spritesheet.png");
    let adult_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 1, None, Some(UVec2::new(0, 33)));
    loaded_assets.adult_atlas = texture_atlas_layouts.add(adult_layout);

    loaded_assets.tabby_sprite = asset_server.load("cats/tabby.png");
    loaded_assets.threat_sprites = CAT_VARIATION_ASSETS
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();
    let threat_running_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 2, None, Some(UVec2::new(640, 100)));
    let threat_walking_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, Some(UVec2::new(384, 100)));
    loaded_assets.threat_running_atlas = texture_atlas_layouts.add(threat_running_layout);
    loaded_assets.threat_walking_atlas = texture_atlas_layouts.add(threat_walking_layout);

    loaded_assets.heart_sprite_sheet = asset_server.load("heart_sprite_sheet.png");
    let heart_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    loaded_assets.heart_atlas_layout = texture_atlas_layouts.add(heart_layout);

    loaded_assets.food_sprites = FOOD_SPRITES
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();
}

pub fn system_spawn_boids(mut commands: Commands, loaded_assets: Res<LoadedAssets>) {
    for _ in 0..10 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_boid(position, &mut commands, &loaded_assets);
    }
}

pub fn system_spawn_threats(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut current_threats: ResMut<CurrentThreats>,
) {
    for index in 0..2 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_threat(
            position,
            &mut commands,
            &loaded_assets,
            &mut current_threats,
            index == 0,
        );
    }
}

#[derive(Resource, Default)]
pub struct CurrentThreats(i32);
pub fn system_continuous_threat_spawning(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut current_threats: ResMut<CurrentThreats>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    duck_query: Query<Entity, (With<Boid>, Without<threat_boid::Threat>)>,
) {
    let window = if let Ok(window) = window_query.get_single() {
        window
    } else {
        return;
    };
    let window_width = window.width();
    let buffer = 50.0;

    let duck_count = duck_query.iter().count();
    if current_threats.0 * 15 < duck_count as i32 {
        let random_position_on_circle =
            Vec2::new(rand::random::<f32>().cos(), rand::random::<f32>().sin())
                * (window_width * 0.5 + buffer);
        spawn_threat(
            random_position_on_circle,
            &mut commands,
            &loaded_assets,
            &mut current_threats,
            false,
        );
    }
}

pub fn spawn_boid(position: Vec2, commands: &mut Commands, loaded_assets: &Res<LoadedAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: loaded_assets.egg_sprite.clone(),
            transform: Transform {
                translation: position.extend(-1.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(life_cycles::Egg)
        .insert(life_cycles::LifeCycleTime(
            rand::random::<f32>()
                * (life_cycles::EGG_HATCH_TIME_MAX - life_cycles::EGG_HATCH_TIME_MIN)
                + life_cycles::EGG_HATCH_TIME_MIN,
        ))
        .insert(sprite_animation::AnimationIndices {
            first: 0,
            last: 0,
            paused: false,
        })
        .insert(sprite_animation::AnimationTimer(Timer::from_seconds(
            0.25,
            TimerMode::Once,
        )))
        .insert(TextureAtlas::default());
}

const CAT_VARIATION_ASSETS: [&str; 4] = [
    "cats/black_4.png",
    "cats/brown_1.png",
    "cats/brown_3.png",
    "cats/brown_7.png",
];

pub fn spawn_threat(
    position: Vec2,
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    current_threats: &mut ResMut<CurrentThreats>,
    tabby: bool,
) {
    current_threats.0 += 1;
    let walking_animation_indices = sprite_animation::AnimationIndices {
        first: 0,
        last: 3,
        paused: false,
    };
    let random_animation_start_index = rand::random::<usize>() % 4;
    let random_animation_timer: f32 = rand::random::<f32>() * 0.5 + 1.0;
    let random_index = rand::random::<usize>() % CAT_VARIATION_ASSETS.len();

    let texture = if tabby {
        loaded_assets.tabby_sprite.clone()
    } else {
        loaded_assets.threat_sprites[random_index].clone()
    };
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                flip_x: true,
                ..Default::default()
            },
            texture,
            transform: Transform {
                translation: position.extend(5.0),
                scale: Vec3::splat(3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(threat_boid::Threat::default())
        .insert(movement::Velocity::default())
        .insert(movement::VelocityLimits {
            min: 0.0,
            max: 200.0,
        })
        .insert((
            TextureAtlas {
                layout: loaded_assets.threat_walking_atlas.clone(),
                index: random_animation_start_index,
            },
            walking_animation_indices,
            sprite_animation::AnimationTimer(Timer::from_seconds(
                random_animation_timer,
                TimerMode::Repeating,
            )),
        ));
}

const FOOD_SPRITES: [&str; 4] = [
    "foods/bread.png",
    "foods/pretzel.png",
    "foods/cake.png",
    "foods/croissant.png",
];

pub fn spawn_food(position: Vec2, commands: &mut Commands, loaded_assets: &Res<LoadedAssets>) {
    let random_index = rand::random::<usize>() % FOOD_SPRITES.len();

    commands
        .spawn(SpriteBundle {
            texture: loaded_assets.food_sprites[random_index].clone(),
            transform: Transform {
                translation: position.extend(-2.0),
                scale: Vec3::splat(1.15),
                rotation: Quat::from_rotation_z(rand::random::<f32>() * std::f32::consts::PI),
            },
            ..Default::default()
        })
        .insert(Food);
}

pub fn spawn_relationship_sprite(
    entity: Entity,
    position: Vec2,
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
) {
    let animation_indices = sprite_animation::AnimationIndices {
        first: 0,
        last: 3,
        paused: false,
    };
    commands.entity(entity).insert((
        SpriteBundle {
            transform: Transform {
                translation: position.extend(5.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            texture: loaded_assets.heart_sprite_sheet.clone(),
            ..default()
        },
        TextureAtlas {
            layout: loaded_assets.heart_atlas_layout.clone(),
            index: animation_indices.first,
        },
        animation_indices,
        sprite_animation::AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
    ));
}
