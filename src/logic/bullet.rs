use crate::logic::walls::Wall;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub const BULLETSIZE: f32 = 10.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets)
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
        bullet_transform.translation += bullet.direction.extend(0.0) * time.delta_seconds() * 400.0;
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
                Vec2::new(BULLETSIZE, BULLETSIZE),
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
