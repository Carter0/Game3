use crate::logic::bullet::Bullet;
use crate::logic::bullet::BULLETSIZE;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;

pub struct PlayerPlugin;

const PLAYERSIZE: f32 = 40.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(shoot)
            .add_system(look_at_cursor);
    }
}

#[derive(Component)]
pub struct Player {
    // Speed is never negative
    speed: f32,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYERSIZE, PLAYERSIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 300.0 });
}

// Move the player with WASD or the arrow keys
fn move_player(
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (player, mut transform) = player_query
        .get_single_mut()
        .expect("A single player was not found");

    let up = keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);
    let down = keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);
    let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

    // If left is pressed than it will be -1, right 1, both they cancel out.
    // This indicates the direction of the player using a unit vector
    let x_axis: i8 = -(left as i8) + right as i8;
    let y_axis: i8 = -(down as i8) + up as i8;
    let move_delta: Vec2 = Vec2::new(x_axis as f32, y_axis as f32);

    // This controls the rate at which the player moves
    let delta_time = time.delta_seconds();
    transform.translation.x += move_delta.x * player.speed * delta_time;
    transform.translation.y += move_delta.y * player.speed * delta_time;
}

// The player always faces the cursor
fn look_at_cursor(windows: Res<Windows>, mut player_query: Query<&mut Transform, With<Player>>) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    // cursor is inside the window, position given
    if let Some(position) = window.cursor_position() {
        // The position of the cursor is given from (0,0) in the top left to (screen width, screen height) in the bottom right.
        // Most bevy coordinates are done with (0,0) in the middle of the screen.
        // This translates from screen space to world space.
        let world_space_cursor_vec = position + Vec2::new(-WINDOWWIDTH / 2.0, -WINDOWHEIGHT / 2.0);

        let mut player_transform = player_query
            .get_single_mut()
            .expect("Could not find a single player");

        // Get the vector from the player to the cursor in 2D and normalize it.
        let to_cursor =
            (world_space_cursor_vec - player_transform.translation.truncate()).normalize();

        // Get the quaternion to rotate the player to the cursor.
        // The player is facing up.
        let rotate_to_cursor = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.));

        // Rotate the enemy to face the player.
        player_transform.rotation = rotate_to_cursor;
    }
}

// The player shoots with space
fn shoot(
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Could not find a single player");

    if keyboard_input.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        let player_translation = player_transform.translation;

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLETSIZE, BULLETSIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(player_translation.x, player_translation.y, 0.0)
                    .with_rotation(player_transform.rotation),
                ..Default::default()
            })
            .insert(Bullet {
                direction: player_transform.local_y().truncate(),
            });
    }
}
