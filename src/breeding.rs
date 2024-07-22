use crate::duck_boid::CloseAdults;
use crate::life_cycles::Adult;
use crate::spawning;
use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component)]
pub struct Relationship {
    pub partner1: Entity,
    pub partner2: Entity,
    pub duration: f32,
}
impl Default for Relationship {
    fn default() -> Self {
        Self {
            partner1: Entity::PLACEHOLDER,
            partner2: Entity::PLACEHOLDER,
            duration: 0.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Sambo {
    pub relationship_entity: Entity,
}

pub fn system_build_relationships(
    mut commands: Commands,
    mut query: Query<(Entity, &CloseAdults), (With<Adult>, Without<Sambo>)>,
) {
    let mut present_entities: HashSet<Entity> = Default::default();
    for (entity, _) in query.iter() {
        present_entities.insert(entity);
    }

    let mut covered_entities: HashSet<Entity> = Default::default();
    for (entity, close_adults) in query.iter_mut() {
        if close_adults.0.is_empty() {
            continue;
        }
        if covered_entities.contains(&entity) {
            continue;
        }
        let other_entity = close_adults.0[0].1;
        if !present_entities.contains(&other_entity) {
            continue;
        }
        covered_entities.insert(other_entity);

        let relationship_entity = commands
            .spawn(Relationship {
                partner1: entity,
                partner2: other_entity,
                ..Default::default()
            })
            .insert(BreedingProgress::default())
            .id();

        commands.entity(entity).try_insert(Sambo {
            relationship_entity,
        });
        commands.entity(other_entity).try_insert(Sambo {
            relationship_entity,
        });
    }
}

const VISIBLE_RELATIONSHIP_THRESHOLD: f32 = 0.5;
const DISBAND_RELATIONSHIP_DISTANCE_2: f32 = 100.0 * 100.0;
const HEART_HEIGHT_OFFSET: f32 = 25.0;
pub fn system_update_relationships(
    time: Res<Time>,
    mut commands: Commands,
    mut relationship_query: Query<
        (Entity, &mut Relationship, Option<&mut Transform>),
        (Without<Adult>, Without<Sambo>),
    >,
    sambo_query: Query<(&Transform, &CloseAdults), (With<Adult>, With<Sambo>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, mut relationship, transform) in relationship_query.iter_mut() {
        relationship.duration += time.delta_seconds();

        let partner1_valid = sambo_query.get(relationship.partner1).is_ok();
        let partner2_valid = sambo_query.get(relationship.partner2).is_ok();
        if !partner1_valid || !partner2_valid {
            commands.entity(entity).despawn();
            if partner1_valid {
                commands.entity(relationship.partner1).remove::<Sambo>();
            }
            if partner2_valid {
                commands.entity(relationship.partner2).remove::<Sambo>();
            }
            continue;
        }

        let partner1_transform = sambo_query.get(relationship.partner1).unwrap().0;
        let partner2_transform = sambo_query.get(relationship.partner2).unwrap().0;

        if (partner1_transform.translation.xy() - partner2_transform.translation.xy())
            .length_squared()
            > DISBAND_RELATIONSHIP_DISTANCE_2
        {
            commands.entity(entity).despawn();
            if partner1_valid {
                commands.entity(relationship.partner1).remove::<Sambo>();
            }
            if partner2_valid {
                commands.entity(relationship.partner2).remove::<Sambo>();
            }
            continue;
        }

        let relationship_position =
            (partner1_transform.translation.xy() + partner2_transform.translation.xy()) / 2.0
                + Vec2::Y * HEART_HEIGHT_OFFSET;

        if relationship.duration >= VISIBLE_RELATIONSHIP_THRESHOLD && transform.is_none() {
            spawning::spawn_relationship_sprite(
                entity,
                relationship_position,
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
            );
        } else if transform.is_some() {
            transform.unwrap().translation = relationship_position.extend(5.0);
        }
    }
}

#[derive(Component, Default)]
pub struct BreedingProgress(pub f32);

pub const BREEDING_DURATION: f32 = 10.0 * crate::life_cycles::TIME_FACTOR;

pub fn system_breeding(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut relationship_query: Query<(&Transform, &mut BreedingProgress), With<Relationship>>,
) {
    for (transform, mut breeding_progress) in relationship_query.iter_mut() {
        breeding_progress.0 += time.delta_seconds();

        if breeding_progress.0 >= BREEDING_DURATION {
            spawning::spawn_boid(transform.translation.xy(), &mut commands, &asset_server);

            breeding_progress.0 = 0.0;
        }
    }
}
