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

// TODO everything below should go in its own module, or atleas the collision stuff should
// TODO make sure all this code makes sense to you in the future
// TODO make a shooting component
// TODO make a spawning component
// TODO make a follow player component

// NOTE
// The invariant here is that each entity can only have one kind of collision
// Where you might run into trouble in the future is when you want to have an
// entity collide with one thing in one way, but collide with another thing in another way.
// Which is actually going to be a problem haha
// But ill get there in the future
// One problem at a time
// This problem might also be solved by relations lol
#[derive(Component)]
enum ColliderType {
    // Reflect the moving object
    Reflect,
    // Stop the moving object
    Stop,
    // Destroy both this object and the colliding object
    // The entity refers to the entity of the other component
    // you need to touch in order for desturuction to happen
    // NOTE this is blocked by relations
    // Destroy(Entity),
    // Does nothing on collision, used for static objects
    Nothing,
}

// Checks whether objects with collision components have collided.
fn check_collisions(
    mut collisions_query: Query<(
        &mut Transform,
        &Sprite,
        Option<&mut Movement>,
        &ColliderType,
    )>,
    time: Res<Time>,
) {
    // The combination is an arrangement of entities's components without repeats
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some(
        [(mut a_transform, a_sprite, a_movement, a_collider_type), (mut b_transform, b_sprite, b_movement, b_collider_type)],
    ) = combinations.fetch_next()
    {
        let a_size: Vec2 = a_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        let b_size: Vec2 = b_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        match (a_movement, b_movement) {
            // Invariant: Only Destroy happens when two moving objects collide (so far)
            (Some(_a_movement), Some(_b_movement)) => {}
            (Some(mut movement), None) => {
                determine_collision(
                    &mut movement.velocity,
                    &time,
                    &mut a_transform,
                    &a_size,
                    &a_collider_type,
                    &b_transform,
                    &b_size,
                );
            }
            (None, Some(mut movement)) => {
                determine_collision(
                    &mut movement.velocity,
                    &time,
                    &mut b_transform,
                    &b_size,
                    &b_collider_type,
                    &a_transform,
                    &a_size,
                );
            }
            (None, None) => {}
        }
    }
}

// Determines whether there a collision will occur in the next frame and then
// delegates to another function that determines what kind of collision response there will be.
fn determine_collision(
    mut velocity: &mut Vec3,
    time: &Time,
    moving_transform: &mut Transform,
    moving_transform_size: &Vec2,
    moving_collider_type: &ColliderType,
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
        match moving_collider_type {
            ColliderType::Nothing => {}
            ColliderType::Stop => {
                stop_moving_entity(
                    &collision,
                    moving_transform,
                    moving_transform_size,
                    static_transform,
                    static_transform_size,
                );
            }
            ColliderType::Reflect => {
                reflect_entity(&collision, &mut velocity);
            }
        }
    }
}

// Reflects the velocity of the entity
fn reflect_entity(collision: &Collision, velocity: &mut Vec3) {
    match collision {
        Collision::Left => *velocity = Vec3::new(velocity.x * -1.0, velocity.y, 0.0),
        Collision::Right => *velocity = Vec3::new(velocity.x * -1.0, velocity.y, 0.0),
        Collision::Top => *velocity = Vec3::new(velocity.x, velocity.y * -1.0, 0.0),
        Collision::Bottom => *velocity = Vec3::new(velocity.x, velocity.y * -1.0, 0.0),
        Collision::Inside => {}
    }
}

// Resets the position of the moving transform to be right before the collision next frame
// transform one is going to be the transform of the moving object
fn stop_moving_entity(
    collision: &Collision,
    moving_transform: &mut Transform,
    moving_transform_size: &Vec2,
    static_transform: &Transform,
    static_transform_size: &Vec2,
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
