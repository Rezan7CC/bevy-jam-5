use bevy::prelude::*;
use crate::movement;

pub fn system_spawn_boids(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let position = Vec2::new(
        rand::random::<f32>() * 800.0 - 400.0,
        rand::random::<f32>() * 600.0 - 300.0,
        );
        spawn_boid(position, &mut commands, &asset_server);
    }
}

fn spawn_boid(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>)
{
    let random_direction = Vec2::new(
        rand::random::<f32>() - 0.5,
        rand::random::<f32>() - 0.5,
    ).normalize();
    let velocity = random_direction * movement::MIN_VELOCITY;

    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        transform: Transform {
            translation: position.extend(0.0),
            scale: Vec3::splat(0.25),
            ..Default::default()
        },
        ..Default::default()
    }).insert(movement::Velocity { 0: velocity });
}