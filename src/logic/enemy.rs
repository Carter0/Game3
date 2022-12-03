use crate::logic::physics::{ColliderType, Movement, ShootingEvent};
use crate::logic::player::{Player, PLAYER_SIZE};
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use rand::Rng;
use std::time::Duration;

pub struct EnemyPlugin;

const NORMAL_ENEMY_SPAWN_TIMESTEP: f64 = 4.0;
const SHOOTING_ENEMY_SPAWN_TIMESTEP: f64 = 7.0;
const SHOOTING_ENEMY_SHOOT_TIMESTEP: f64 = 2.0;
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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SHOOTING_ENEMY_SHOOT_TIMESTEP))
                .with_system(shooting_enemy_shooting),
        )
        .add_system(move_normal_enemies)
        .add_event::<EnemyDeathEvent>()
        .add_system(normal_enemy_player_collisions)
        .add_system(shooting_enemy_player_collisions)
        .add_system(rotate_to_face_player)
        .add_system(spawn_enemies);
    }
}

// TODO speed might be able to be put into movement component somehow
#[derive(Component)]
pub struct Enemy;

// Shooting enemies need a few things
// 1. The ability to shoot
// 3. Shooting enemy should rotate to face the player

#[derive(Component)]
pub struct ShootingEnemy;

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
        .spawn(EnemySpawn {
            spawn_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            enemy_type,
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
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
    // server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, transform, mut enemy_spawn) in &mut enemy_spawn_query {
        // // This line needs to be moved somwhere else
        // let handle: Handle<Image> = server.load("sprites/basic-enemy.png");
        enemy_spawn.spawn_timer.tick(time.delta());

        if enemy_spawn.spawn_timer.finished() {
            commands.entity(entity).despawn();

            match enemy_spawn.enemy_type {
                EnemyType::NormalEnemy => {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                                ..Default::default()
                            },
                            // texture: handle,
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
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
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

// Shooting enemies shoot at the player
fn shooting_enemy_shooting(
    mut event_writer: EventWriter<ShootingEvent>,
    query: Query<Entity, With<ShootingEnemy>>,
) {
    for shooting_enemy_entity in &query {
        event_writer.send(ShootingEvent(shooting_enemy_entity));
    }
}

// TODO abstract out into a rotation component
// TODO use fixed time step or delta time?
// Rotate the shooting enemy so it faces the player
fn rotate_to_face_player(
    mut shooting_enemy_query: Query<&mut Transform, With<ShootingEnemy>>,
    player_query: Query<&Transform, (With<Player>, Without<ShootingEnemy>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Could not find single player");

    for mut shooting_enemy_transform in &mut shooting_enemy_query {
        let direction_to_player =
            (player_transform.translation - shooting_enemy_transform.translation).normalize();

        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, direction_to_player);
        shooting_enemy_transform.rotation = rotate_to_player;
    }
    // Get the normalized vector from the enemy to the player
    // Call Quat::from_rotation_arc using Vec3::Y as the up
    // set the rotation of the shooting enemy
}

// I don't like how I have two identical systems :(
// Enemies kill the player if they touch them
fn normal_enemy_player_collisions(
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<(&Transform, Entity), (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = player_query
        .get_single()
        .expect("Could not find single player");

    for enemy_transform in &enemy_query {
        enemy_player_collisions(
            &mut commands,
            &enemy_transform.translation,
            &player_transform.translation,
            player_entity,
        );
    }
}

// Enemies kill the player if they touch them
fn shooting_enemy_player_collisions(
    enemy_query: Query<&Transform, With<ShootingEnemy>>,
    player_query: Query<(&Transform, Entity), (With<Player>, Without<ShootingEnemy>)>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = player_query
        .get_single()
        .expect("Could not find single player");

    for enemy_transform in &enemy_query {
        enemy_player_collisions(
            &mut commands,
            &enemy_transform.translation,
            &player_transform.translation,
            player_entity,
        );
    }
}

fn enemy_player_collisions(
    commands: &mut Commands,
    enemy_translation: &Vec3,
    player_translation: &Vec3,
    player_entity: Entity,
) {
    if let Some(_collision) = collide(
        *enemy_translation,
        Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
        *player_translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    ) {
        commands.entity(player_entity).despawn();
    }
}
