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
enum RigidBodyType {
    // Static rigid bodies do not move on collision, they are immobile
    Static,
    // Dynamic objects move and react to events in the game
    Dynamic,
}

// Note this needs to happen AFTER movement so that it can reset the position on collision
// create_system_set(fixedtimestep, add_system(movement).label(movement)).add_system(collisions).after(movement)

// TODO query on rigidbody type
fn check_collisions(mut collisions_query: Query<(&mut Transform, &Sprite, &RigidBodyType)>) {
    // The combination is an arrangement of entities's components without repeats
    let mut combinations = collisions_query.iter_combinations_mut();

    while let Some(
        [(mut a_transform, a_sprite, a_rigidbody_type), (mut b_transform, b_sprite, b_rigidbody_type)],
    ) = combinations.fetch_next()
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
            match (a_rigidbody_type, b_rigidbody_type) {
                // TODO fix collision bug
                // player is flickering
                // TODO maybe just stop velocity instead of pushing back??
                // Remember not all objects have a movement
                // throw some print statements down and run it a bunch???
                (RigidBodyType::Dynamic, RigidBodyType::Dynamic) => {}
                (RigidBodyType::Dynamic, RigidBodyType::Static) => match collision {
                    Collision::Left => a_transform.translation += Vec3::new(b_size.x, 0.0, 0.0),
                    Collision::Right => a_transform.translation -= Vec3::new(b_size.x, 0.0, 0.0),
                    Collision::Top => a_transform.translation -= Vec3::new(0.0, b_size.y, 0.0),
                    Collision::Bottom => a_transform.translation += Vec3::new(0.0, b_size.y, 0.0),
                    Collision::Inside => {}
                },
                (RigidBodyType::Static, RigidBodyType::Dynamic) => match collision {
                    Collision::Left => b_transform.translation += Vec3::new(a_size.x, 0.0, 0.0),
                    Collision::Right => b_transform.translation -= Vec3::new(a_size.x, 0.0, 0.0),
                    Collision::Top => b_transform.translation -= Vec3::new(0.0, a_size.y, 0.0),
                    Collision::Bottom => b_transform.translation += Vec3::new(0.0, a_size.y, 0.0),
                    Collision::Inside => {}
                },
                (RigidBodyType::Static, RigidBodyType::Static) => {}
            }
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
