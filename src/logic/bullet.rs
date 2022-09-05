use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets);
    }
}

// TODO
// maybe think of adding a vector for the velocity instead of a speed
// it could be negative, normalized, and then also describe the direction
// the bullet is going.
#[derive(Component)]
pub struct Bullet {
    // Speed is never negative
    pub speed: f32,
}

// TODO this is incorrect because it is not moving on the right transform
fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in &mut bullet_query {
        let normalized = bullet_transform.translation.normalize();
        println!("Y component is {}", normalized.y);
        bullet_transform.translation.y += bullet.speed * time.delta_seconds();
    }
}
