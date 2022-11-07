use crate::logic::physics::{ColliderType, Movement};
use crate::logic::player::{Player, PLAYER_SIZE};
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use rand::Rng;
use std::time::Duration;

pub struct EnemyPlugin;

const NORMAL_ENEMY_SPAWN_TIMESTEP: f64 = 3.0;
const SHOOTING_ENEMY_SPAWN_TIMESTEP: f64 = 5.0;
pub const ENEMY_SIZE: f32 = 40.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: it might make more sense for these two to be on a normal timer instead of on fixed time step
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(NORMAL_ENEMY_SPAWN_TIMESTEP))
                .with_system(spawn_normal_enemy),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SHOOTING_ENEMY_SPAWN_TIMESTEP))
                .with_system(spawn_shooting_enemy),
        )
        .add_system(move_normal_enemies)
        .add_event::<EnemyDeathEvent>()
        .add_system(enemy_player_collisions)
        .add_system(spawn_enemies);
    }
}

// TODO speed might be able to be put into movement component somehow
#[derive(Component)]
pub struct Enemy;

// Shooting enemies need a few things
// 1. The ability to shoot
// 2. Player collisions should destroy, really wished relations existed :(
// 3. Shooting enemy should rotate to face the player

#[derive(Component)]
struct ShootingEnemy;

// Shows the location of where an enemy is going to spawn in
#[derive(Component)]
struct EnemySpawn {
    // The timer here refers to how long it takes for the enemy to spawn in
    spawn_timer: Timer,
    // The type of enemy that is going to spawn in
    enemy_type: EnemyType,
}

enum EnemyType {
    ShootingEnemy,
    NormalEnemy,
}

fn spawn_shooting_enemy(mut commands: Commands) {
    spawn_enemy_location(&mut commands, EnemyType::ShootingEnemy);
}

fn spawn_normal_enemy(mut commands: Commands) {
    spawn_enemy_location(&mut commands, EnemyType::NormalEnemy);
}

fn spawn_enemy_location(commands: &mut Commands, enemy_type: EnemyType) {
    let mut rng = rand::thread_rng();

    let (x, y) = (
        WINDOWWIDTH / 2.0 - rng.gen_range(40.0..=WINDOWWIDTH - 40.0),
        WINDOWHEIGHT / 2.0 - rng.gen_range(40.0..=WINDOWHEIGHT - 40.0),
    );

    commands
        .spawn()
        .insert(EnemySpawn {
            spawn_timer: Timer::new(Duration::from_secs(3), false),
            enemy_type,
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
    mut enemy_spawn_query: Query<(Entity, &Transform, &mut EnemySpawn)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut enemy_spawn) in &mut enemy_spawn_query {
        enemy_spawn.spawn_timer.tick(time.delta());

        // TODO spawn a shooting enemy
        if enemy_spawn.spawn_timer.finished() {
            commands.entity(entity).despawn();

            match enemy_spawn.enemy_type {
                EnemyType::NormalEnemy => {
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
                        .insert(Enemy)
                        .insert(Movement {
                            velocity: Vec3::new(0.0, 0.0, 0.0),
                        });
                }
                EnemyType::ShootingEnemy => {
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::ORANGE_RED,
                                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                                ..Default::default()
                            },
                            transform: *transform,
                            ..Default::default()
                        })
                        .insert(ShootingEnemy);
                }
            }
        }
    }
}

pub struct EnemyDeathEvent {
    pub death_position: Vec3,
}

// Enemies follow the player.
fn move_normal_enemies(
    mut enemy_query: Query<(&Transform, &mut Movement), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Could not find a single player.");

    for (enemy_transform, mut movement) in &mut enemy_query {
        let vector_to_player = player_transform.translation;
        let vector_to_enemy = enemy_transform.translation;
        let direction_from_enemy_to_player: Vec3 = (vector_to_player - vector_to_enemy).normalize();

        // This works by changing the velocity of the enemy every tick
        // to be in the direction of the player
        // 200.0 is the speed of the enemy
        movement.velocity = direction_from_enemy_to_player * 200.0;
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
