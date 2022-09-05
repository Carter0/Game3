use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands) {
    // The ceiling
    let ceiling_size_x = WINDOWWIDTH;
    let ceiling_size_y = 40.0;

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(ceiling_size_x, ceiling_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, WINDOWHEIGHT / 2.0, 1.0),
        ..Default::default()
    });

    // The floor
    let floor_size_x = WINDOWWIDTH;
    let floor_size_y = 40.0;

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(floor_size_x, floor_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0, 1.0),
        ..Default::default()
    });

    // The Left Wall
    let left_wall_size_x = 40.0;
    let left_wall_size_y = WINDOWHEIGHT;
    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(left_wall_size_x, left_wall_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-WINDOWWIDTH / 2.0, 0.0, 1.0),
        ..Default::default()
    });

    // The Right Wall
    let right_wall_size_x = 40.0;
    let right_wall_size_y = WINDOWHEIGHT;
    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(right_wall_size_x, right_wall_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(WINDOWWIDTH / 2.0, 0.0, 1.0),
        ..Default::default()
    });
}
