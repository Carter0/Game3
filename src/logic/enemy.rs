use crate::logic::physics::ColliderType;
use crate::logic::player::{Player, PLAYER_SIZE};
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use rand::Rng;
use std::time::Duration;

pub struct EnemyPlugin;

const ENEMY_SPAWN_TIMESTEP: f64 = 3.0;
pub const ENEMY_SIZE: f32 = 40.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(ENEMY_SPAWN_TIMESTEP))
                .with_system(generate_spawn_coordinates),
        )
        .add_system(move_enemies)
        .add_event::<EnemyDeathEvent>()
        .add_system(enemy_player_collisions)
        .add_system(spawn_enemies);
    }
}

#[derive(Component)]
pub struct Enemy {
    // Speed is always positive
    speed: f32,
}

// Shows the location of where an enemy is going to spawn in
#[derive(Component)]
struct EnemySpawnLocation {
    spawn_timer: Timer,
}

// Run this every enemy spawn timestep
fn generate_spawn_coordinates(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let (x, y) = (
        WINDOWWIDTH / 2.0 - rng.gen_range(40.0..=WINDOWWIDTH - 40.0),
        WINDOWHEIGHT / 2.0 - rng.gen_range(40.0..=WINDOWHEIGHT - 40.0),
    );

    commands
        .spawn()
        .insert(EnemySpawnLocation {
            spawn_timer: Timer::new(Duration::from_secs(3), false),
        })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec2::new(x, y).extend(0.0)),
            ..Default::default()
        })
        .insert(ColliderType::Stop);
}

// Spawn enemies once the enemy spawn timer is up
fn spawn_enemies(
    mut enemy_spawn_query: Query<(Entity, &Transform, &mut EnemySpawnLocation)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut enemy_spawns) in &mut enemy_spawn_query {
        enemy_spawns.spawn_timer.tick(time.delta());

        if enemy_spawns.spawn_timer.finished() {
            commands.entity(entity).despawn();

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::MAROON,
                        custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                        ..Default::default()
                    },
                    transform: *transform,
                    ..Default::default()
                })
                .insert(Enemy { speed: 200.0 });
        }
    }
}

pub struct EnemyDeathEvent {
    pub death_position: Vec3,
}

// Enemies follow the player.
fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Could not find a single player.");

    for (mut enemy_transform, enemy) in &mut enemy_query {
        let vector_to_player = player_transform.translation;
        let vector_to_enemy = enemy_transform.translation;
        let direction_from_enemy_to_player = (vector_to_player - vector_to_enemy).normalize();

        enemy_transform.translation +=
            direction_from_enemy_to_player * enemy.speed * time.delta_seconds();
    }
}

// Enemies kill the player if they touch them
fn enemy_player_collisions(
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<(&Transform, Entity), (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = player_query
        .get_single()
        .expect("Could not find single player");

    for enemy_transform in &enemy_query {
        if let Some(_collision) = collide(
            enemy_transform.translation,
            Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        ) {
            commands.entity(player_entity).despawn();
        }
    }
}
