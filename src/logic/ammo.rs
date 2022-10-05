use crate::logic::enemy::EnemyDeathEvent;
use bevy::prelude::*;

pub struct AmmoPlugin;

pub const BULLET_WIDTH: f32 = 10.0;
pub const BULLET_HEIGHT: f32 = 30.0;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ammo);
    }
}

#[derive(Component)]
pub struct Ammo;

// Ammo spawns from dead enemies
fn spawn_ammo(mut enemy_death_events: EventReader<EnemyDeathEvent>, mut commands: Commands) {
    for enemy_death_event in enemy_death_events.iter() {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_WIDTH, BULLET_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_translation(enemy_death_event.death_position),
                ..Default::default()
            })
            .insert(Ammo);
    }
}

// For now ammo is just a number in the bottom right
// TODO make the ammo UI look nicer
fn ammo_ui() {}
