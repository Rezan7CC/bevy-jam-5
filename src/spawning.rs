use crate::food::Food;
use crate::{life_cycles, movement, sprite_animation, threat_boid};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LoadedAssets {
    egg_sprite: Handle<Image>,

    pub duckling_sprite: Handle<Image>,
    pub duckling_atlas: Handle<TextureAtlasLayout>,

    pub juvenile_sprite: Handle<Image>,
    pub juvenile_atlas: Handle<TextureAtlasLayout>,

    pub adult_sprite: Handle<Image>,
    pub adult_atlas: Handle<TextureAtlasLayout>,

    threat_sprite: Handle<Image>,

    heart_sprite_sheet: Handle<Image>,
    heart_atlas_layout: Handle<TextureAtlasLayout>,

    food_sprites: Vec<Handle<Image>>,
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut loaded_assets: ResMut<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    loaded_assets.egg_sprite = asset_server.load("egg.png");

    loaded_assets.duckling_sprite = asset_server.load("ducks/ducky-walk.png");
    let duckling_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 1, None, None);
    loaded_assets.duckling_atlas = texture_atlas_layouts.add(duckling_layout);

    loaded_assets.juvenile_sprite = asset_server.load("ducks/ducky-walk.png");
    let juvenile_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 1, None, None);
    loaded_assets.juvenile_atlas = texture_atlas_layouts.add(juvenile_layout);

    loaded_assets.adult_sprite = asset_server.load("ducks/ducky-walk.png");
    let adult_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 1, None, None);
    loaded_assets.adult_atlas = texture_atlas_layouts.add(adult_layout);

    loaded_assets.threat_sprite = asset_server.load("threat.png");

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

pub fn system_spawn_threats(mut commands: Commands, loaded_assets: Res<LoadedAssets>) {
    for _ in 0..2 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_threat(position, &mut commands, &loaded_assets);
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
        .insert(sprite_animation::AnimationIndices { first: 0, last: 0 })
        .insert(sprite_animation::AnimationTimer(Timer::from_seconds(
            0.25,
            TimerMode::Once,
        )))
        .insert(TextureAtlas::default());
}

pub fn spawn_threat(position: Vec2, commands: &mut Commands, loaded_assets: &Res<LoadedAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: loaded_assets.threat_sprite.clone(),
            transform: Transform {
                translation: position.extend(5.0),
                scale: Vec3::splat(0.25),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(threat_boid::Threat::default())
        .insert(movement::Velocity::default())
        .insert(movement::VelocityLimits {
            min: 0.0,
            max: 200.0,
        });
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
                scale: Vec3::splat(1.0),
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
    let animation_indices = sprite_animation::AnimationIndices { first: 0, last: 3 };
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
