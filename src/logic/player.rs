use crate::logic::ammo::{Ammo, BULLET_HEIGHT, BULLET_WIDTH};
use crate::logic::bullet::Bullet;
use crate::logic::bullet::BULLET_SIZE;
use crate::logic::walls::Wall;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct PlayerPlugin;

pub const PLAYER_SIZE: f32 = 40.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(shoot)
            .add_system(player_wall_collisions)
            .add_system(collect_ammo)
            .add_system(look_at_cursor);
    }
}

#[derive(Component)]
pub struct Player {
    // Speed is never negative
    speed: f32,
    ammo: u8,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 300.0,
            ammo: 3,
        });
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
        let mut player_transform = player_query
            .get_single_mut()
            .expect("Could not find a single player");

        // The position of the cursor is given from (0,0) in the top left to (screen width, screen height) in the bottom right.
        // Most bevy coordinates are done with (0,0) in the middle of the screen.
        // This translates from screen space to world space.
        let world_space_cursor_vec = position + Vec2::new(-WINDOWWIDTH / 2.0, -WINDOWHEIGHT / 2.0);

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
// The bullet starts moving in the direction the player is facing.
fn shoot(
    mut player_query: Query<(&Transform, &mut Player)>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) {
    let (player_transform, mut player) = player_query
        .get_single_mut()
        .expect("Could not find a single player");

    if keyboard_input.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        let player_translation = player_transform.translation;

        // The player cannot shoot if they have no ammunition
        if player.ammo > 0 {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                        ..Default::default()
                    },
                    // Scale the local y unit vector so that the bullet does not
                    // immediately collide with the player.
                    transform: Transform::from_translation(
                        player_translation + player_transform.local_y() * 50.0,
                    ),
                    ..Default::default()
                })
                .insert(Bullet {
                    direction: player_transform.local_y().truncate(),
                });

            player.ammo -= 1;
        }
    }
}

// The player cannot move outside the arena
fn player_wall_collisions(
    mut player_query: Query<&mut Transform, With<Player>>,
    walls_query: Query<(&Transform, &Wall), Without<Player>>,
) {
    let mut player_transform = player_query
        .get_single_mut()
        .expect("Could not find a single player");

    for (wall_transform, wall) in &walls_query {
        if let Some(collision) = collide(
            wall_transform.translation,
            Vec2::new(wall.width, wall.height),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        ) {
            match collision {
                Collision::Left => {
                    player_transform.translation = Vec3::new(
                        -WINDOWWIDTH / 2.0 + wall.width,
                        player_transform.translation.y,
                        0.0,
                    );
                }
                Collision::Right => {
                    player_transform.translation = Vec3::new(
                        WINDOWWIDTH / 2.0 - wall.width,
                        player_transform.translation.y,
                        0.0,
                    );
                }
                Collision::Top => {
                    player_transform.translation = Vec3::new(
                        player_transform.translation.x,
                        WINDOWHEIGHT / 2.0 - wall.height,
                        0.0,
                    );
                }
                Collision::Bottom => {
                    player_transform.translation = Vec3::new(
                        player_transform.translation.x,
                        -WINDOWHEIGHT / 2.0 + wall.height,
                        0.0,
                    );
                }
                Collision::Inside => {}
            }
        }
    }
}

// Player collects the ammo by passing over it
fn collect_ammo(
    mut player_query: Query<(&Transform, &mut Player)>,
    ammo_query: Query<(&Transform, Entity), (With<Ammo>, Without<Player>)>,
    mut commands: Commands,
) {
    let (player_transform, mut player) = player_query
        .get_single_mut()
        .expect("Could not find a single player");

    for (ammo_transform, ammo_entity) in &ammo_query {
        if let Some(_collision) = collide(
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            ammo_transform.translation,
            Vec2::new(BULLET_WIDTH, BULLET_HEIGHT),
        ) {
            commands.entity(ammo_entity).despawn();

            player.ammo += 1;

            // The player cannot have more than 10 bullets
            if player.ammo > 10 {
                player.ammo = 10;
            }
        }
    }
}
