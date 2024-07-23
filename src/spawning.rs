use crate::food::Food;
use crate::{life_cycles, movement, sprite_animation, threat_boid};
use bevy::prelude::*;

pub fn system_spawn_boids(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_boid(position, &mut commands, &asset_server);
    }
}

pub fn system_spawn_threats(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..2 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_threat(position, &mut commands, &asset_server);
    }
}

pub fn spawn_boid(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("egg.png"),
            transform: Transform {
                translation: position.extend(-1.0),
                scale: Vec3::splat(0.25),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(life_cycles::Egg)
        .insert(life_cycles::LifeCycleTime(
            rand::random::<f32>()
                * (life_cycles::EGG_HATCH_TIME_MAX - life_cycles::EGG_HATCH_TIME_MIN)
                + life_cycles::EGG_HATCH_TIME_MIN,
        ));
}

pub fn spawn_threat(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("threat.png"),
            transform: Transform {
                translation: position.extend(5.0),
                scale: Vec3::splat(0.25),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(threat_boid::Threat)
        .insert(movement::Velocity::default())
        .insert(movement::VelocityLimits {
            min: 0.0,
            max: 200.0,
        });
}

const FOOD_SPRITES: [&str; 5] = [
    "foods/bread.png",
    "foods/pretzel.png",
    "foods/cake.png",
    "foods/croissant.png",
    "foods/donut.png",
];

pub fn spawn_food(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let sprite_path = FOOD_SPRITES[rand::random::<usize>() % FOOD_SPRITES.len()];

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(sprite_path),
            transform: Transform {
                translation: position.extend(-2.0),
                scale: Vec3::splat(1.0),
                rotation: Quat::from_rotation_z(rand::random::<f32>() * std::f32::consts::PI),
            },
            ..Default::default()
        })
        .insert(Food);
}

pub fn switch_sprite(
    sprite_path: &'static str,
    asset_server: &Res<AssetServer>,
    entity_image: &mut Handle<Image>,
) {
    let new_image_handle = asset_server.load(sprite_path);
    *entity_image = new_image_handle;
}

pub fn spawn_relationship_sprite(
    entity: Entity,
    position: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("heart_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = sprite_animation::AnimationIndices { first: 0, last: 3 };
    commands.entity(entity).insert((
        SpriteBundle {
            transform: Transform {
                translation: position.extend(5.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        sprite_animation::AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
    ));
}
