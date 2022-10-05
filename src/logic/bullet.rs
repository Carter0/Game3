use crate::logic::enemy::{Enemy, EnemyDeathEvent, ENEMY_SIZE};
use crate::logic::player::{Player, PLAYER_SIZE};
use crate::logic::walls::Wall;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub const BULLET_SIZE: f32 = 20.0;
const BULLET_SPEED: f32 = 400.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets)
            .add_system(bullet_enemy_collisions)
            .add_system(bullet_player_collisions)
            .add_system(bullet_wall_collisions);
    }
}

#[derive(Component)]
pub struct Bullet {
    // Direction is the normalized local y vector of the player
    pub direction: Vec2,
}

// Bullets move forward at a constant velocity
fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in &mut bullet_query {
        bullet_transform.translation +=
            bullet.direction.extend(0.0) * time.delta_seconds() * BULLET_SPEED;
    }
}

// Bullets bounce off the walls
fn bullet_wall_collisions(
    mut bullet_query: Query<(&Transform, &mut Bullet)>,
    wall_query: Query<(&Transform, &Wall), Without<Bullet>>,
) {
    for (bullet_transform, mut bullet) in &mut bullet_query {
        for (wall_transform, wall) in &wall_query {
            if let Some(collision) = collide(
                wall_transform.translation,
                Vec2::new(wall.width, wall.height),
                bullet_transform.translation,
                Vec2::new(BULLET_SIZE, BULLET_SIZE),
            ) {
                match collision {
                    Collision::Left => {
                        bullet.direction = Vec2::new(bullet.direction.x * -1.0, bullet.direction.y)
                    }
                    Collision::Right => {
                        bullet.direction = Vec2::new(bullet.direction.x * -1.0, bullet.direction.y)
                    }
                    Collision::Top => {
                        bullet.direction = Vec2::new(bullet.direction.x, bullet.direction.y * -1.0)
                    }
                    Collision::Bottom => {
                        bullet.direction = Vec2::new(bullet.direction.x, bullet.direction.y * -1.0)
                    }
                    Collision::Inside => {}
                }
            }
        }
    }
}

// When the bullet hits an enemy destroy both the
// bullet and the enemy.
fn bullet_enemy_collisions(
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<Enemy>)>,
    mut add_to_score: EventWriter<EnemyDeathEvent>,
    mut commands: Commands,
) {
    for (enemy_transform, enemy_entity) in &enemy_query {
        for (bullet_transform, bullet_entity) in &bullet_query {
            if let Some(_collision) = collide(
                bullet_transform.translation,
                Vec2::new(BULLET_SIZE, BULLET_SIZE),
                enemy_transform.translation,
                Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            ) {
                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();
                add_to_score.send(EnemyDeathEvent {
                    death_position: bullet_transform.translation,
                });
            }
        }
    }
}

// When the bullet hits the player destroy both the
// bullet and the player.
fn bullet_player_collisions(
    player_query: Query<(&Transform, Entity), With<Player>>,
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<Player>)>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = player_query
        .get_single()
        .expect("Could not find single player");

    for (bullet_transform, bullet_entity) in &bullet_query {
        if let Some(_collision) = collide(
            bullet_transform.translation,
            Vec2::new(BULLET_SIZE, BULLET_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        ) {
            commands.entity(player_entity).despawn();
            commands.entity(bullet_entity).despawn();
        }
    }
}
