use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;

pub struct EnemyPlugin;

const ENEMY_SPAWN_TIMESTEP: f64 = 3.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(ENEMY_SPAWN_TIMESTEP))
                .with_system(spawn_enemies),
        );
    }
}

enum SpawnSide {
    Left,
    Right,
    Up,
    Down,
}

fn get_spawn_side(integer: i8) -> SpawnSide {
    match integer {
        1 => SpawnSide::Left,
        2 => SpawnSide::Right,
        3 => SpawnSide::Up,
        _ => SpawnSide::Down,
    }
}

// TODO debug this nonsense lol
// NOTE enemy spawn time will need to be configured based on feedback
// Enemies spawn randomly on the sides of the arena every 3 seconds.
fn spawn_enemies(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let spawn_side = get_spawn_side(rng.gen_range(1..=1));

    let spawn_position: Vec2 = match spawn_side {
        SpawnSide::Left => Vec2::new(-WINDOWWIDTH / 2.0 + 40.0, rng.gen_range(1.0..=WINDOWHEIGHT)),
        SpawnSide::Right => Vec2::new(WINDOWWIDTH / 2.0 - 40.0, rng.gen_range(1.0..=WINDOWHEIGHT)),
        SpawnSide::Up => Vec2::new(WINDOWHEIGHT / 2.0, rng.gen_range(1.0..=WINDOWWIDTH)),
        SpawnSide::Down => Vec2::new(-WINDOWHEIGHT / 2.0, rng.gen_range(1.0..=WINDOWWIDTH)),
    };

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::MAROON,
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(spawn_position.extend(0.0)),
        ..Default::default()
    });
}
