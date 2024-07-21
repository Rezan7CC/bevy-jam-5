use crate::food::Food;
use crate::life_cycles;
use bevy::prelude::*;

pub fn system_spawn_boids(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..100 {
        let position = Vec2::new(
            rand::random::<f32>() * 800.0 - 400.0,
            rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_boid(position, &mut commands, &asset_server);
    }
}

pub fn spawn_boid(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("egg.png"),
            transform: Transform {
                translation: position.extend(0.0),
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
                translation: position.extend(0.0),
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
