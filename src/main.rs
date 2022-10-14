use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

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
        .add_system(check_collisions)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}

#[derive(Component)]
struct Collidable;

// Alrighty a few things with this, cause I now realize you are implementing general collision,
// which might be a whole long process by itself. I'm honestly sorta down for that, but this just got more
// complicated, again lol.

// Firstly, all the walls are colliding with themselves off the bat. And the way you have the collision
// logic set up is making it so the bottom and top walls are moving to some random spot.
// Your problem is that you don't know which transform is a and which one is b.
// You need to know that in order for your logic to make any sense haha

// I think you need to rethink what happens when two entities collide.
fn check_collisions(mut collisions_query: Query<(&mut Transform, &Sprite), With<Collidable>>) {
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some([(mut a_transform, a_sprite), (b_transform, b_sprite)]) =
        combinations.fetch_next()
    {
        let a_size: Vec2 = a_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        let b_size: Vec2 = b_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        if let Some(collision) = collide(
            a_transform.translation,
            a_size,
            b_transform.translation,
            b_size,
        ) {
            // TODO figure out what you want to happen here
            match collision {
                Collision::Left => {}
                Collision::Right => {}
                Collision::Top => {}
                Collision::Bottom => {}
                Collision::Inside => {}
            }
        }
    }
}

// TODO player wall collisions into a general collision component

// Perhaps this is not quite the right component.
// Maybe the move is to use this more like a empty tag component
// #[derive(Component)]
// struct CollisionBetween {
//     a_transform: Transform,
//     a_size: Vec2,
//     b_transform: Transform,
//     b_size: Vec2,
// }

// Then down here, perhaps a good idea is to have something like
// Query<(&mut Transform, &Sprite), With<Collidable>>)
// You could do this but you would need to do that twice?
// Like for the other object you want to collide with
// But then you would need to make sure that you don't hit yourself again
// Like you don't want a to collide with a
// fn check_collisions(mut collisions_query: Query<&CollisionBetween>) {
//     for CollisionBetween {
//         mut a_transform,
//         a_size,
//         b_transform,
//         b_size,
//     } in &mut collisions_query
//     {
//         if let Some(collision) = collide(
//             a_transform.translation,
//             *a_size,
//             b_transform.translation,
//             *b_size,
//         ) {
//             match collision {
//                 Collision::Left => {
//                     a_transform.translation = Vec3::new(
//                         b_transform.translation.x + b_size.x / 2.0,
//                         a_transform.translation.y,
//                         0.0,
//                     );
//                 }
//                 Collision::Right => {
//                     a_transform.translation = Vec3::new(
//                         b_transform.translation.x - b_size.x / 2.0,
//                         a_transform.translation.y,
//                         0.0,
//                     );
//                 }
//                 Collision::Top => {
//                     a_transform.translation = Vec3::new(
//                         a_transform.translation.x,
//                         b_transform.translation.y + b_size.y / 2.0,
//                         0.0,
//                     );
//                 }
//                 Collision::Bottom => {
//                     a_transform.translation = Vec3::new(
//                         a_transform.translation.x,
//                         b_transform.translation.y - b_size.y / 2.0,
//                         0.0,
//                     );
//                 }
//                 Collision::Inside => {}
//             }
//         }
//     }
// }
