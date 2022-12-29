use crate::logic::physics::ColliderType;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;

pub struct WallsPlugin;

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

    for spawn_location in create_spawn_locations() {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                // color: Color::PURPLE,
                custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
                ..Default::default()
            },
            texture: horizontal_wall_handle.clone(),
            transform: Transform::from_translation(spawn_location.extend(0.0)),
            ..Default::default()
        };

        commands.spawn(sprite_bundle).insert(ColliderType::Nothing);
    }
    // let sprite_bundle = SpriteBundle {
    //     sprite: Sprite {
    //         // color: Color::PURPLE,
    //         custom_size: Some(Vec2::new(BLOCKSIZE as f32, BLOCKSIZE as f32)),
    //         ..Default::default()
    //     },
    //     texture: horizontal_wall_handle.clone(),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     ..Default::default()
    // };

    // commands.spawn(sprite_bundle);
}

// Getting the positions of the blocks is tricky because I want to remove the corners.
// So I need to get the right number of blocks and then move them over so they align correctly on the screen
fn create_spawn_locations() -> Vec<Vec2> {
    // The corners are there own sprites so I don't want them in this iterator.
    let blocks_per_width = WINDOWWIDTH as i16 / BLOCKSIZE - 2;
    // let blocks_per_height = WINDOWHEIGHT as i16 / BLOCKSIZE;

    let horizontal_block_positions: Vec<i16> = (1..=blocks_per_width)
        .map(|x| WINDOWWIDTH as i16 - (x * BLOCKSIZE))
        .map(|x| x - get_edge_of_screen(WINDOWWIDTH) - BLOCKSIZE / 2)
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

// fn spawn_walls(mut commands: Commands) {
//     // The ceiling
//     let ceiling_size_x = WINDOWWIDTH;
//     let ceiling_size_y = 40.0;

//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(10.0, 70.0, 70.0),
//                 custom_size: Some(Vec2::new(ceiling_size_x, ceiling_size_y)),
//                 ..Default::default()
//             },
//             transform: Transform::from_xyz(0.0, WINDOWHEIGHT / 2.0, 1.0),
//             ..Default::default()
//         })
//         .insert(ColliderType::Nothing);

//     // The floor
//     let floor_size_x = WINDOWWIDTH;
//     let floor_size_y = 40.0;

//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(10.0, 70.0, 70.0),
//                 custom_size: Some(Vec2::new(floor_size_x, floor_size_y)),
//                 ..Default::default()
//             },
//             transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0, 1.0),
//             ..Default::default()
//         })
//         .insert(ColliderType::Nothing);

//     // The Left Wall
//     let left_wall_size_x = 40.0;
//     let left_wall_size_y = WINDOWHEIGHT;
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(10.0, 70.0, 70.0),
//                 custom_size: Some(Vec2::new(left_wall_size_x, left_wall_size_y)),
//                 ..Default::default()
//             },
//             transform: Transform::from_xyz(-WINDOWWIDTH / 2.0, 0.0, 1.0),
//             ..Default::default()
//         })
//         .insert(ColliderType::Nothing);

//     // The Right Wall
//     let right_wall_size_x = 40.0;
//     let right_wall_size_y = WINDOWHEIGHT;
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(10.0, 70.0, 70.0),
//                 custom_size: Some(Vec2::new(right_wall_size_x, right_wall_size_y)),
//                 ..Default::default()
//             },
//             transform: Transform::from_xyz(WINDOWWIDTH / 2.0, 0.0, 1.0),
//             ..Default::default()
//         })
//         .insert(ColliderType::Nothing);
// }
