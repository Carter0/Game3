use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::time::FixedTimestep;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

// Run 60 frames per second
const FIXED_TIMESTEP: f64 = 1.0 / 60.0;

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

// TODO make something like this...
// enum CollisionType {
//     Reflect
//     Stop
//     Destroy
// }

#[derive(Component)]
struct Collider;

// #[derive(Component)]
// enum RigidBodyType {
//     // Static rigid bodies do not move on collision, they are immobile
//     Static,
//     // Dynamic objects move and react to events in the game
//     Dynamic,
// }

fn check_collisions(
    mut collisions_query: Query<(&mut Transform, &Sprite, Option<&Movement>), With<Collider>>,
    time: Res<Time>,
) {
    // The combination is an arrangement of entities's components without repeats
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some(
        [(mut a_transform, a_sprite, a_movement), (mut b_transform, b_sprite, b_movement)],
    ) = combinations.fetch_next()
    {
        // So I sorta want to use this chunk of data for two different things here
        // 1. I wanna know the position next frame
        // 2. I wanna know which object is the one that is moving and which one is the one that is staying still
        // Honestly I think they should just be two separe chunks of data
        // I want A to be the one that is moving
        // let a_position = match a_movement {
        //     Some(movement) => movement.velocity + a_transform.translation,
        //     None => a_transform.translation,
        // };

        // let b_position = match b_movement {
        //     Some(movement) => movement.velocity + b_transform.translation,
        //     None => b_transform.translation,
        // };

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
                let position_next_frame =
                    movement.velocity * time.delta_seconds() + a_transform.translation;

                println!("In V1");

                // For future me: The A and B thing is confusing because the official docs use the same
                // terminology for the collision_aabb method that I use in this system.
                // The only reason I am doing this is to always know that I am colliding with
                // the non-moving objects sides. So I am colliding with the non moving objects left side for example.

                // The return value is the side of `B` that `A` has collided with. `Left` means that
                // `A` collided with `B`'s left side. `Top` means that `A` collided with `B`'s top side.
                // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                // If all sides are involved, `Inside` is returned.
                println!("A next position is {}", position_next_frame.x);
                println!("Current b position is {}", b_transform.translation.x);

                if let Some(collision) =
                    collide(position_next_frame, a_size, b_transform.translation, b_size)
                {
                    // TODO recalculate the position to move based on where the collision is going to happen
                    // So basically
                    // If the desired movement is going to cause a collision next frame
                    // Then recalculate the movement for the next frame to be the distance right before the collision

                    println!("In V1 collision");

                    match collision {
                        Collision::Left => {
                            println!("collision occured on the left side of the right wall");
                            let b_x_pos: f32 = b_transform.translation.x - b_size.x / 2.0;
                            let a_cur_x_pos: f32 = a_transform.translation.x + a_size.x / 2.0;

                            println!("B position is {b_x_pos}");
                            println!("A position is {a_cur_x_pos}");

                            a_transform.translation.x =
                                (b_x_pos - a_cur_x_pos) + a_transform.translation.x;
                        }
                        Collision::Right => {}
                        Collision::Top => {}
                        Collision::Bottom => {}
                        Collision::Inside => {}
                    }
                }
            }
            (None, Some(movement)) => {
                let position_next_frame = movement.velocity + b_transform.translation;

                // The return value is the side of `B` that `A` has collided with. `Left` means that
                // `A` collided with `B`'s left side. `Top` means that `A` collided with `B`'s top side.
                // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                // If all sides are involved, `Inside` is returned.
                if let Some(collision) =
                    collide(position_next_frame, b_size, a_transform.translation, a_size)
                {
                    // TODO recalculate the position to move based on where the collision is going to happen
                    // So basically
                    // If the desired movement is going to cause a collision next frame
                    // Then recalculate the movement for the next frame to be the distance right before the collision

                    match collision {
                        Collision::Left => {
                            println!("In V2");
                            // println!("collision occured on the left side of the right wall 222");
                            // // THis is so confusing lol
                            // let b_x_pos: f32 = a_transform.translation.x - a_size.x / 2.0;
                            // let a_cur_x_pos: f32 = b_transform.translation.x + b_size.x / 2.0;

                            // println!("B2 position is {b_x_pos}");
                            // println!("A2 position is {a_cur_x_pos}");

                            // b_transform.translation.x = b_x_pos - a_cur_x_pos;
                        }
                        Collision::Right => {}
                        Collision::Top => {}
                        Collision::Bottom => {}
                        Collision::Inside => {}
                    }
                }
            }
            (None, None) => {}
        }

        // There is going to be a collision next frame
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
