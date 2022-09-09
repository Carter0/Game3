use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;
use std::fmt;

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

impl fmt::Display for SpawnSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpawnSide::Left => write!(f, "Left"),
            SpawnSide::Right => write!(f, "Right"),
            SpawnSide::Up => write!(f, "Up"),
            SpawnSide::Down => write!(f, "Down"),
        }
    }
}

fn get_spawn_side(integer: i8) -> SpawnSide {
    match integer {
        1 => SpawnSide::Left,
        2 => SpawnSide::Right,
        3 => SpawnSide::Up,
        _ => SpawnSide::Down,
    }
}

// NOTE enemy spawn time will need to be configured based on feedback
// Enemies spawn randomly on the sides of the arena every 3 seconds.
fn spawn_enemies(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let spawn_side = get_spawn_side(rng.gen_range(1..=4));

    let spawn_position: Vec2 = match spawn_side {
        SpawnSide::Left => Vec2::new(
            -WINDOWWIDTH / 2.0 + 40.0,
            rng.gen_range(-WINDOWHEIGHT / 2.0..=WINDOWHEIGHT / 2.0),
        ),
        SpawnSide::Right => Vec2::new(
            WINDOWWIDTH / 2.0 - 40.0,
            rng.gen_range(-WINDOWHEIGHT / 2.0..=WINDOWHEIGHT / 2.0),
        ),
        SpawnSide::Up => Vec2::new(
            rng.gen_range(-WINDOWWIDTH / 2.0..=WINDOWWIDTH / 2.0),
            WINDOWHEIGHT / 2.0 - 40.0,
        ),
        SpawnSide::Down => Vec2::new(
            rng.gen_range(-WINDOWWIDTH / 2.0..=WINDOWWIDTH / 2.0),
            -WINDOWHEIGHT / 2.0 + 40.0,
        ),
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
