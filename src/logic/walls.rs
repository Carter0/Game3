use crate::logic::physics::ColliderType;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;

pub struct WallsPlugin;

// TODO My game now lags depending on how many bullets are flying around

// NOTE
// The size of the blocks matters because I need the corner sprites to match the edge of the screen.
// This means that the blocks size needs to be a multiple of the width and height of the screen.
// NOTE
// Making this any smaller makes the game lag. I suspect its the collision system because I have no broad phase collision detection
const BLOCKSIZE: i16 = 40;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls_v2);
    }
}

#[derive(Component)]
pub struct Wall {
    pub width: f32,
    pub height: f32,
}

fn spawn_walls_v2(mut commands: Commands, server: Res<AssetServer>) {
    let horizontal_wall_handle: Handle<Image> = server.load("sprites/wall-horizontal.png");
    let vertical_wall_handle: Handle<Image> = server.load("sprites/wall-vertical.png");
    let top_left_wall_handle: Handle<Image> = server.load("sprites/wall-corner-top-left.png");
    let top_right_wall_handle: Handle<Image> = server.load("sprites/wall-corner-top-right.png");
    let bottom_left_wall_handle: Handle<Image> = server.load("sprites/wall-corner-bottom-left.png");
    let bottom_right_wall_handle: Handle<Image> =
        server.load("sprites/wall-corner-bottom-right.png");

    for spawn_location in create_horizontal_spawn_locations() {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
                ..Default::default()
            },
            texture: horizontal_wall_handle.clone(),
            transform: Transform::from_translation(spawn_location.extend(0.0)),
            ..Default::default()
        };

        commands.spawn(sprite_bundle).insert(ColliderType::Nothing);
    }

    for spawn_location in create_vertical_spawn_locations() {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
                ..Default::default()
            },
            texture: vertical_wall_handle.clone(),
            transform: Transform::from_translation(spawn_location.extend(0.0)),
            ..Default::default()
        };

        commands.spawn(sprite_bundle).insert(ColliderType::Nothing);
    }

    let top_left_wall = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
            ..Default::default()
        },
        texture: top_left_wall_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -WINDOWWIDTH / 2.0 + (BLOCKSIZE / 2) as f32,
            WINDOWHEIGHT / 2.0 - (BLOCKSIZE / 2) as f32,
            0.0,
        )),
        ..Default::default()
    };

    let top_right_wall = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
            ..Default::default()
        },
        texture: top_right_wall_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            WINDOWWIDTH / 2.0 - (BLOCKSIZE / 2) as f32,
            WINDOWHEIGHT / 2.0 - (BLOCKSIZE / 2) as f32,
            0.0,
        )),
        ..Default::default()
    };

    let bottom_left_wall = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
            ..Default::default()
        },
        texture: bottom_left_wall_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -WINDOWWIDTH / 2.0 + (BLOCKSIZE / 2) as f32,
            -WINDOWHEIGHT / 2.0 + (BLOCKSIZE / 2) as f32,
            0.0,
        )),
        ..Default::default()
    };

    let bottom_right_wall = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
            ..Default::default()
        },
        texture: bottom_right_wall_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            WINDOWWIDTH / 2.0 - (BLOCKSIZE / 2) as f32,
            -WINDOWHEIGHT / 2.0 + (BLOCKSIZE / 2) as f32,
            0.0,
        )),
        ..Default::default()
    };

    commands.spawn(top_left_wall).insert(ColliderType::Nothing);
    commands.spawn(top_right_wall).insert(ColliderType::Nothing);
    commands
        .spawn(bottom_left_wall)
        .insert(ColliderType::Nothing);
    commands
        .spawn(bottom_right_wall)
        .insert(ColliderType::Nothing);
}

// Getting the positions of the blocks is tricky because I want to remove the corners.
// So I need to get the right number of blocks and then move them over so they align correctly on the screen
// TODO I'm fairly sure this code can be cleaned up somehow. To make it less confusing.
fn create_vertical_spawn_locations() -> Vec<Vec2> {
    let blocks_per_height = WINDOWHEIGHT as i16 / BLOCKSIZE - 2;

    let vertical_block_positions: Vec<i16> = (1..=blocks_per_height)
        .map(|y| y * BLOCKSIZE)
        .map(|y| y - get_edge_of_screen(WINDOWHEIGHT) + BLOCKSIZE / 2)
        .collect();

    let vertical_blocks_left: Vec<Vec2> = vertical_block_positions
        .clone()
        .into_iter()
        .map(|y| {
            Vec2::new(
                (get_edge_of_screen(-WINDOWWIDTH) + BLOCKSIZE / 2) as f32,
                y as f32,
            )
        })
        .collect();

    let vertical_blocks: Vec<Vec2> = vertical_block_positions
        .clone()
        .into_iter()
        .map(|x| {
            Vec2::new(
                (get_edge_of_screen(WINDOWWIDTH) - BLOCKSIZE / 2) as f32,
                x as f32,
            )
        })
        .chain(vertical_blocks_left)
        .collect();

    vertical_blocks
}

fn create_horizontal_spawn_locations() -> Vec<Vec2> {
    // The corners are there own sprites so I don't want them in this iterator.
    let blocks_per_width = WINDOWWIDTH as i16 / BLOCKSIZE - 2;

    let horizontal_block_positions: Vec<i16> = (1..=blocks_per_width)
        .map(|x| x * BLOCKSIZE)
        .map(|x| x - get_edge_of_screen(WINDOWWIDTH) + BLOCKSIZE / 2)
        .collect();

    let horizontal_blocks_top: Vec<Vec2> = horizontal_block_positions
        .clone()
        .into_iter()
        .map(|x| {
            Vec2::new(
                x as f32,
                (get_edge_of_screen(WINDOWHEIGHT) - BLOCKSIZE / 2) as f32,
            )
        })
        .collect();

    let horizontal_blocks: Vec<Vec2> = horizontal_block_positions
        .clone()
        .into_iter()
        .map(|x| {
            Vec2::new(
                x as f32,
                (get_edge_of_screen(-WINDOWHEIGHT) + BLOCKSIZE / 2) as f32,
            )
        })
        .chain(horizontal_blocks_top)
        .collect();

    horizontal_blocks
}

// The edge of the screen is half of the total size
fn get_edge_of_screen(window_size: f32) -> i16 {
    window_size as i16 / 2
}
