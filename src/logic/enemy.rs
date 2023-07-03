use crate::logic::physics::{ColliderType, Movement, ShootingEvent};
use crate::logic::player::{Player, PLAYER_SIZE};
use crate::{EnemySprite, Flashing, TurretSprite, WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;
use std::time::Duration;

pub struct EnemyPlugin;

// const NORMAL_ENEMY_SPAWN_TIMESTEP: f64 = 4.0;
// const SHOOTING_ENEMY_SPAWN_TIMESTEP: f64 = 7.0;
// const SHOOTING_ENEMY_SHOOT_TIMESTEP: f64 = 2.0;
pub const ENEMY_SIZE: f32 = 40.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemy_spawning)
            .add_system(tick_timers)
            .add_system(spawn_normal_enemy.run_if(normal_enemy_spawn_system))
            .add_system(spawn_shooting_enemy.run_if(shooting_enemy_spawn_system))
            .add_system(shooting_enemy_shooting)
            .add_system(move_normal_enemies)
            .add_event::<EnemyDeathEvent>()
            .add_system(normal_enemy_player_collisions)
            .add_system(shooting_enemy_player_collisions)
            .add_system(rotate_to_face_player)
            .add_system(spawn_enemies);
    }
}

// TODO I can probs combine these timers into one somehow
// I bet im being a dumb dumb
#[derive(Component)]
struct NormalSpawnTimer {
    // The timer refers to how long it takes for each
    // enemy spawn system to run
    timer: Timer,
}

#[derive(Component)]
struct ShootingSpawnTimer {
    // The timer refers to how long it takes for each
    // enemy spawn system to run
    timer: Timer,
}

// TODO idk this is odd
fn tick_timers(
    time: Res<Time>,
    mut query: Query<(&mut NormalSpawnTimer, &mut ShootingSpawnTimer)>,
) {
    for (mut normal_timer, mut shooting_timer) in &mut query {
        normal_timer.timer.tick(time.delta());
        shooting_timer.timer.tick(time.delta());
    }
}
// on startup, setup enemy spawning
fn setup_enemy_spawning(mut commands: Commands) {
    commands
        .spawn(NormalSpawnTimer {
            timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
        })
        .insert(ShootingSpawnTimer {
            timer: Timer::new(Duration::from_secs(7), TimerMode::Repeating),
        });
}

fn normal_enemy_spawn_system(query: Query<&NormalSpawnTimer>) -> bool {
    let spawn_timer = query
        .get_single()
        .expect("Can only be one normal spawn timer");

    spawn_timer.timer.finished()
}

fn shooting_enemy_spawn_system(query: Query<&ShootingSpawnTimer>) -> bool {
    let spawn_timer = query
        .get_single()
        .expect("Can only be one normal spawn timer");

    spawn_timer.timer.finished()
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ShootingEnemy {
    // Timer for how long it takes for an enemy to shoot
    shooting_timer: Timer,
}

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

fn spawn_shooting_enemy(mut commands: Commands, turret_sprite: Res<TurretSprite>) {
    spawn_enemy_location(
        &mut commands,
        EnemyType::ShootingEnemy,
        turret_sprite.0.clone(),
    );
}

fn spawn_normal_enemy(mut commands: Commands, enemy_sprite: Res<EnemySprite>) {
    spawn_enemy_location(
        &mut commands,
        EnemyType::NormalEnemy,
        enemy_sprite.0.clone(),
    );
}

fn spawn_enemy_location(commands: &mut Commands, enemy_type: EnemyType, sprite: Handle<Image>) {
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
                color: Color::CRIMSON,
                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                ..Default::default()
            },
            texture: sprite,
            transform: Transform::from_translation(Vec2::new(x, y).extend(0.0)),
            ..Default::default()
        })
        .insert(Flashing {
            flashed: false,
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
        })
        .insert(ColliderType::Stop);
}

// Spawn enemies once the enemy spawn timer is up
fn spawn_enemies(
    mut enemy_spawn_query: Query<(Entity, &Transform, &mut EnemySpawn)>,
    mut commands: Commands,
    enemy_sprite: Res<EnemySprite>,
    turret_sprite: Res<TurretSprite>,
    time: Res<Time>,
) {
    for (entity, transform, mut enemy_spawn) in &mut enemy_spawn_query {
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
                            texture: enemy_sprite.0.clone(),
                            transform: *transform,
                            ..Default::default()
                        })
                        .insert(Enemy)
                        .insert(Movement {
                            velocity: Vec3::new(0.0, 0.0, 0.0),
                        })
                        .insert(FacingPlayer);
                }
                EnemyType::ShootingEnemy => {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                                ..Default::default()
                            },
                            texture: turret_sprite.0.clone(),
                            transform: *transform,
                            ..Default::default()
                        })
                        .insert(ShootingEnemy {
                            shooting_timer: Timer::new(
                                Duration::from_secs(2),
                                TimerMode::Repeating,
                            ),
                        })
                        .insert(FacingPlayer);
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
    mut query: Query<(Entity, &mut ShootingEnemy)>,
    time: Res<Time>,
) {
    // TODO stop using events
    for (shooting_enemy_entity, mut shooting_enemy) in &mut query {
        if shooting_enemy.shooting_timer.tick(time.delta()).finished() {
            event_writer.send(ShootingEvent(shooting_enemy_entity));
        }
    }
}

#[derive(Component)]
struct FacingPlayer;

fn rotate_to_face_player(
    mut facing_player_query: Query<&mut Transform, With<FacingPlayer>>,
    player_query: Query<&Transform, (With<Player>, Without<FacingPlayer>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Could not find single player");

    for mut facing_transform in &mut facing_player_query {
        let direction_to_player =
            (player_transform.translation - facing_transform.translation).normalize();

        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, direction_to_player);
        facing_transform.rotation = rotate_to_player;
    }
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
