use crate::logic::ammo::{Ammo, BULLET_HEIGHT, BULLET_WIDTH};
use crate::logic::physics::ShootingEvent;
use crate::logic::physics::{ColliderType, Movement};
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

pub const PLAYER_SIZE: f32 = 40.0;
pub const STARTING_AMMO: u8 = 3;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_keyboard_input)
            .add_system(shoot)
            .add_system(collect_ammo)
            .add_system(look_at_cursor);
    }
}

#[derive(Component)]
pub struct Player {
    // Ammo is limited to some amount (I forget how much)
    pub ammo: u8,
}

fn spawn_player(mut commands: Commands, server: Res<AssetServer>) {
    let player_size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE);
    let handle: Handle<Image> = server.load("sprites/player.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player_size),
                ..Default::default()
            },
            texture: handle,
            ..Default::default()
        })
        .insert(Player {
            ammo: STARTING_AMMO,
        })
        .insert(Movement {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ColliderType::Stop);
}

// Move the player with WASD or the arrow keys
fn player_keyboard_input(
    mut player_query: Query<&mut Movement, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut movement = player_query
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

    movement.velocity = Vec3::new(move_delta.x * 400.0, move_delta.y * 400.0, 0.0);
}

// The player always faces the cursor
fn look_at_cursor(windows: Query<&Window>, mut player_query: Query<&mut Transform, With<Player>>) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.single();

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
fn shoot(
    mut player_query: Query<(Entity, &mut Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut event_writer: EventWriter<ShootingEvent>,
) {
    let (player_entity, mut player) = player_query
        .get_single_mut()
        .expect("Could not find a single player");

    if keyboard_input.just_pressed(KeyCode::Space) || buttons.just_pressed(MouseButton::Left) {
        // The player cannot shoot if they have no ammunition
        if player.ammo > 0 {
            event_writer.send(ShootingEvent(player_entity));
            player.ammo -= 1;
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
