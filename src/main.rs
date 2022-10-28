use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::time::FixedTimestep;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

// Run 80 frames per second
const FIXED_TIMESTEP: f64 = 1.0 / 80.0;

mod logic;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "TBD".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::walls::WallsPlugin)
        .add_plugin(logic::bullet::BulletPlugin)
        .add_plugin(logic::enemy::EnemyPlugin)
        .add_plugin(logic::score::ScorePlugin)
        .add_plugin(logic::ammo::AmmoPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP))
                .with_system(move_transforms)
                // Will override moving the transforms if a collision occurs
                .with_system(check_collisions.after(move_transforms)),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}

// TODO I think you should make an event for this
// Something kind of like an on collision event
#[derive(Component)]
enum ColliderType {
    // Reflect the moving object
    // Reflect,
    // Stop the moving object
    Stop,
    // Destroy both this object and the colliding object
    // Destroy,
    // Does nothing on collision, used for static objects
    Nothing,
}

// Checks whether objects with collision components have collided.
// TODO this can't be a with anymore, need to bring it into the query
fn check_collisions(
    mut collisions_query: Query<(&mut Transform, &Sprite, Option<&Movement>), With<ColliderType>>,
    time: Res<Time>,
) {
    // The combination is an arrangement of entities's components without repeats
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some(
        [(mut a_transform, a_sprite, a_movement), (mut b_transform, b_sprite, b_movement)],
    ) = combinations.fetch_next()
    {
        let a_size: Vec2 = a_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        let b_size: Vec2 = b_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        // Invariant: I am only checking collisions between objects thats move and objects that do not
        match (a_movement, b_movement) {
            (Some(_a_movement), Some(_b_movement)) => {}
            (Some(movement), None) => {
                reset_colliding_object(
                    &movement.velocity,
                    &time,
                    &mut a_transform,
                    &a_size,
                    &b_transform,
                    &b_size,
                );
            }
            (None, Some(movement)) => {
                reset_colliding_object(
                    &movement.velocity,
                    &time,
                    &mut b_transform,
                    &b_size,
                    &a_transform,
                    &a_size,
                );
            }
            (None, None) => {}
        }
    }
}

// Resets the position of the moving transform to be right before the collision next frame
// transform one is going to be the transform of the moving object
fn reset_colliding_object(
    velocity: &Vec3,
    time: &Time,
    moving_transform: &mut Transform,
    moving_transform_size: &Vec2,
    static_transform: &Transform,
    static_transform_size: &Vec2,
) {
    let position_next_frame = *velocity * time.delta_seconds() + moving_transform.translation;

    // The return value is the side of `B` that `A` has collided with. `Left` means that
    // `A` collided with `B`'s left side. `Top` means that `A` collided with `B`'s top side.
    // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
    // If all sides are involved, `Inside` is returned.

    if let Some(collision) = collide(
        position_next_frame,
        *moving_transform_size,
        static_transform.translation,
        *static_transform_size,
    ) {
        match collision {
            Collision::Left => {
                // a => entity a from the combinations query
                // b => entity b from the combinations query
                // one => collision entity one from the collisions method
                // two => collision entity two from the collisions method

                let one_x_pos = static_transform.translation.x - static_transform_size.x / 2.0;
                let two_cur_x_pos = moving_transform.translation.x + moving_transform_size.x / 2.0;
                moving_transform.translation.x =
                    (one_x_pos - two_cur_x_pos) + moving_transform.translation.x;
            }
            Collision::Right => {
                let one_x_pos = static_transform.translation.x + static_transform_size.x / 2.0;
                let two_cur_x_pos = moving_transform.translation.x - moving_transform_size.x / 2.0;

                moving_transform.translation.x =
                    (one_x_pos - two_cur_x_pos) + moving_transform.translation.x;
            }
            Collision::Top => {
                let one_y_pos = static_transform.translation.y + static_transform_size.y / 2.0;
                let two_cur_y_pos = moving_transform.translation.y - moving_transform_size.y / 2.0;

                moving_transform.translation.y =
                    (one_y_pos - two_cur_y_pos) + moving_transform.translation.y;
            }
            Collision::Bottom => {
                let one_y_pos = static_transform.translation.y - static_transform_size.y / 2.0;
                let two_cur_y_pos = moving_transform.translation.y + moving_transform_size.y / 2.0;

                moving_transform.translation.y =
                    (one_y_pos - two_cur_y_pos) + moving_transform.translation.y;
            }
            Collision::Inside => {}
        }
    }
}

#[derive(Component)]
struct Movement {
    velocity: Vec3,
}

// Does fairly basic linear movement
fn move_transforms(mut query: Query<(&mut Transform, &Movement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.velocity * time.delta_seconds();
    }
}
