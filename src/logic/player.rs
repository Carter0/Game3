use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
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
                custom_size: Some(Vec2::new(40.0, 40.0)),
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

// TODO finish this later, see trig video
// The player always faces the cursor
fn look_at_cursor(windows: Res<Windows>, mut player_query: Query<&mut Transform, With<Player>>) {

    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    // cursor is inside the window, position given
    if let Some(position) = window.cursor_position() {

        let player_transform = player_query.get_single_mut().expect("Could not find a single player");

    }

}
