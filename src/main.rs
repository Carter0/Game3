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

#[derive(Component)]
struct Movement {
    velocity: Vec2,
}

// TODO make something like this...
// enum CollisionType {
//     Reflect
//     Stop
//     Destroy)
// }

// TODO make something like this...
// enum RigidBodyType {
//     Static
//     Dynamic
// }

// Note this needs to happen AFTER movement so that it can reset the position on collision
// create_system_set(fixedtimestep, add_system(movement).label(movement)).add_system(collisions).after(movement)

// TODO query on rigidbody type
fn check_collisions(mut collisions_query: Query<(&mut Transform, &Sprite), With<Collidable>>) {
    // The combination is an arrangement of entities's components without repeats
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some([(a_transform, a_sprite), (b_transform, b_sprite)]) = combinations.fetch_next() {
        let a_size: Vec2 = a_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        let b_size: Vec2 = b_sprite
            .custom_size
            .expect("All sprites need to have custom sizes.");

        // I think the reason you are getting tripped up here is because you have different types of rigid bodies that can have collisions
        // Like the walls are static objects that don't move whereas the player is a dynamic object that can.
        // NOTE I am not sure I have the terminology right here, is it static/kinematic???
        // I think if two movement/dynamic object collide then they should reflect
        // OR should two dynamic objects not collide at all?

        // TODO create an enum for the type of objects colliding
        if let Some(_collision) = collide(
            a_transform.translation,
            a_size,
            b_transform.translation,
            b_size,
        ) {
            // TODO have an enum here later for the collision type
        }
    }
}

// TODO make a movement system
// Does fairly basic linear movement
