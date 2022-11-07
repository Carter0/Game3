use crate::logic::enemy::{Enemy, EnemyDeathEvent, ShootingEnemy, ENEMY_SIZE};
use crate::logic::player::{Player, PLAYER_SIZE};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub const BULLET_SIZE: f32 = 20.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_normal_enemy_collisions)
            .add_system(bullet_shooting_enemy_collisions)
            .add_system(bullet_player_collisions);
    }
}

// The tag component for the bullet
#[derive(Component)]
pub struct Bullet;

// TODO find some way to abstract this :(
// When the bullet hits an enemy destroy both the
// bullet and the enemy.
fn bullet_normal_enemy_collisions(
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

// When the bullet hits an enemy destroy both the
// bullet and the enemy.
fn bullet_shooting_enemy_collisions(
    enemy_query: Query<(&Transform, Entity), With<ShootingEnemy>>,
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<ShootingEnemy>)>,
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
