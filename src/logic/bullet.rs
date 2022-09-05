use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets);
    }
}

#[derive(Component)]
pub struct Bullet {
    // Direction is the normalized local y vector of the player
    pub direction: Vec2,
}

fn move_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in &mut bullet_query {
        bullet_transform.translation += bullet.direction.extend(0.0) * time.delta_seconds() * 400.0;
    }
}
