use crate::logic::bullet::{Bullet, BULLET_SIZE};
use crate::BulletSprite;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use itertools::Itertools;
use std::cmp::Ordering;

pub struct PhysicsPlugin;

const FIXED_TIMESTEP: f32 = 1.0 / 60.0;

// #[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
// struct FixedTimeSet;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems_to_schedule(
            CoreSchedule::FixedUpdate,
            (
                move_transforms,
                detect_collisions,
                apply_system_buffers,
                reflect_entity,
                stop_moving_entity,
                apply_system_buffers,
            )
                .chain(),
        )
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .add_event::<ShootingEvent>()
        .add_system(shoot);
    }
}
// NOTE
// The invariant here is that each entity can only have one kind of collision
// Where you might run into trouble in the future is when you want to have an
// entity collide with one thing in one way, but collide with another thing in another way.
// Which is actually going to be a problem haha
// But ill get there in the future
// One problem at a time
// This problem might also be solved by relations lol
#[derive(Component)]
pub enum ColliderType {
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

struct ColliderInfo<'a> {
    transform: &'a Transform,
    sprite: &'a Sprite,
    collider_type: &'a ColliderType,
    movement: Option<&'a Movement>,
    entity: Entity,
}

// This is the specific kind of ordering I want for sort and sweep.
fn interval_ordering(col_info1: &ColliderInfo, col_info2: &ColliderInfo) -> Ordering {
    let interval1_start =
        col_info1.transform.translation.x - col_info1.sprite.custom_size.unwrap().x / 2.0;

    let interval2_start =
        col_info2.transform.translation.x - col_info2.sprite.custom_size.unwrap().x / 2.0;

    let interval1_end =
        col_info1.transform.translation.x + col_info1.sprite.custom_size.unwrap().x / 2.0;

    let interval2_end =
        col_info2.transform.translation.x + col_info2.sprite.custom_size.unwrap().x / 2.0;

    if interval1_start < interval2_start {
        return Ordering::Less;
    }

    if interval1_start > interval2_start {
        return Ordering::Greater;
    }

    // start values are equal

    if interval1_end < interval2_end {
        return Ordering::Less;
    }

    if interval1_end > interval2_end {
        return Ordering::Greater;
    }

    Ordering::Equal
}

fn group_by_helper(collider_info_one: &ColliderInfo, collider_info_two: &ColliderInfo) -> bool {
    let interval_one_end = collider_info_one.transform.translation.x
        + collider_info_one.sprite.custom_size.unwrap().x / 2.0;

    let interval_two_start = collider_info_two.transform.translation.x
        - collider_info_two.sprite.custom_size.unwrap().x / 2.0;

    interval_two_start <= interval_one_end
}

fn sort_and_sweep<'a>(collider_infos: &'a mut Vec<ColliderInfo>) -> Vec<&'a [ColliderInfo<'a>]> {
    collider_infos.sort_unstable_by(|collider_info_one, collider_info_two| {
        interval_ordering(collider_info_one, collider_info_two)
    });

    collider_infos
        .group_by(|collider_info_one, collider_info_two| {
            group_by_helper(collider_info_one, collider_info_two)
        })
        .filter(|vec2_slice| vec2_slice.len() != 1)
        .collect::<Vec<&[ColliderInfo]>>()
}

fn aabb_collisions<'a>(
    possibly_colliding_entities: (&'a ColliderInfo, &'a ColliderInfo),
) -> Option<CollisionInfo<'a>> {
    let a_movement = possibly_colliding_entities.0.movement;
    let b_movement = possibly_colliding_entities.1.movement;

    match (a_movement, b_movement) {
        // Invariant: Only Destroy happens when two moving objects collide (so far)
        // NOTE I believe that destroy functionality needs relations
        (Some(_a_movement), Some(_b_movement)) => None,
        (Some(_movement), None) => {
            determine_collision(possibly_colliding_entities.0, possibly_colliding_entities.1)
        }
        (None, Some(_movement)) => {
            determine_collision(possibly_colliding_entities.1, possibly_colliding_entities.0)
        }
        (None, None) => None,
    }
}

// Determines whether there a collision will occur in the next frame and then
// delegates to another function that determines what kind of collision response there will be.
fn determine_collision<'a>(
    moving_collider_info: &'a ColliderInfo,
    static_collider_info: &'a ColliderInfo,
) -> Option<CollisionInfo<'a>> {
    let ColliderInfo {
        transform: moving_transform,
        sprite: moving_sprite,
        collider_type,
        movement: _,
        entity: moving_entity,
    } = moving_collider_info;

    let ColliderInfo {
        transform: static_transform,
        sprite: static_sprite,
        collider_type: _b_collider_type,
        movement: _b_movement,
        entity: static_entity,
    } = static_collider_info;

    let moving_transform_size = moving_sprite.custom_size.unwrap();
    let static_transform_size = static_sprite.custom_size.unwrap();

    // The return value is the side of `B` that `A` has collided with. `Left` means that
    // `A` collided with `B`'s left side. `Top` means that `A` collided with `B`'s top side.
    // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
    // If all sides are involved, `Inside` is returned.

    collide(
        moving_transform.translation,
        moving_transform_size,
        static_transform.translation,
        static_transform_size,
    )
    .and_then(|collision| {
        Some(CollisionInfo {
            collision,
            collider_type,
            moving_entity,
            static_entity,
        })
    })
}

// Checks whether objects with collision components have collided.
fn detect_collisions(
    collisions_query: Query<(
        &Transform,
        &Sprite,
        &ColliderType,
        Option<&Movement>,
        Entity,
    )>,
    mut commands: Commands,
) {
    let mut test = collisions_query
        .iter()
        .map(
            |(transform, sprite, collider_type, movement, entity)| ColliderInfo {
                transform,
                sprite,
                collider_type,
                movement,
                entity,
            },
        )
        .collect::<Vec<ColliderInfo>>();

    let test: Vec<&[ColliderInfo]> = sort_and_sweep(&mut test);

    for possibly_colliding_entities in test {
        let collisions: Vec<CollisionInfo> = possibly_colliding_entities
            .iter()
            .tuple_combinations::<(&ColliderInfo, &ColliderInfo)>()
            .map(|entity_combinations| aabb_collisions(entity_combinations))
            .flatten()
            .collect();

        // Might not need this. Head hurts but im close lol
        for CollisionInfo {
            collision,
            collider_type,
            moving_entity,
            static_entity,
        } in collisions
        {
            println!("collision");
            match collider_type {
                ColliderType::Reflect => {
                    commands
                        .entity(*moving_entity)
                        .insert(Reflection { collision });
                }
                ColliderType::Stop => {
                    commands.entity(*moving_entity).insert(Stop {
                        collision,
                        static_entity: *static_entity,
                    });
                }
                ColliderType::Nothing => {}
            };
        }
    }
}

struct CollisionInfo<'a> {
    collision: Collision,
    collider_type: &'a ColliderType,
    moving_entity: &'a Entity,
    static_entity: &'a Entity,
}

// TODO this component should be a sparseset component
#[derive(Component)]
struct Reflection {
    collision: Collision,
}

// TODO
// All right I can see a few potential bugs from the system you have written.
// Taking a look at the print statements I have thrown around here, I think everything is broadly working
// in that sense that the code you wrote is doing what you expect. The problem is what you expect
// isn't doing what you want.
//
// There are multiple collision checks per each reflection check. Taking a look at the logs you have
// potentiallly 2-4 collision checks per each reflection. Really you want it to be 1-1, so you might be able to solve this through
// system set ordering.
//
// The other potential problem is that while you are reflecting, you aren't going far enough in a frame to be away enough
// from the collision, so you are colliding twice, which is causing another reflection loop, and you are entering into an infinite loop.
// Maybe the solution to this is to provide some kind of buffer for how often a reflection can occur? This could in theory be solved by having
// one movement system run before a collision system. Because then we atleast get ONE movement and that will probs get the object moving in the right direction,
// ie, away from a collision.
//
// Another thought is about commands. On Bevy 0.9, commands are run at the end of the stage, and that means these systems might be
// running at odd times. The way to fix this would be to upgrade to 0.10 LOL, cause then I can schedule when the commands are applied specifically.
// Yeah this is likely one of the problems as well.
//
// Command application is now signaled through an exclusive system called apply_system_buffers. You can add instances of this system anywhere in
// your schedule. If one system depends on the effects of commands from another, make sure an apply_system_buffers appears somewhere between them.
//
// I think a key takeaway from all this is that when and how I run my systems is now important for me to make progress in my game. It is no longer
// practical for me to just say "everything runs in parallel". Practically, I need to get an idea for the ordering of systems in my game.

// Reflects the velocity of the entity
fn reflect_entity(mut query: Query<(&Reflection, &mut Movement, Entity)>, mut commands: Commands) {
    for (reflection, mut movement, entity) in &mut query {
        println!("{:?}", reflection.collision);
        match reflection.collision {
            Collision::Left => {
                movement.velocity = Vec3::new(movement.velocity.x * -1.0, movement.velocity.y, 0.0)
            }
            Collision::Right => {
                movement.velocity = Vec3::new(movement.velocity.x * -1.0, movement.velocity.y, 0.0)
            }
            Collision::Top => {
                movement.velocity = Vec3::new(movement.velocity.x, movement.velocity.y * -1.0, 0.0)
            }
            Collision::Bottom => {
                movement.velocity = Vec3::new(movement.velocity.x, movement.velocity.y * -1.0, 0.0)
            }
            Collision::Inside => {}
        }

        commands.entity(entity).remove::<Reflection>();
    }
}

// TODO this component should be a sparseset component
#[derive(Component)]
struct Stop {
    collision: Collision,
    static_entity: Entity,
}

// Resets the position of the moving transform to be the position right before the collision.
fn stop_moving_entity(
    mut moving_transform_query: Query<(&Stop, &mut Transform, &Sprite, Entity)>,
    static_transform_query: Query<(&Transform, &Sprite, Without<Stop>)>,
    mut commands: Commands,
) {
    for (stop, mut moving_transform, moving_sprite, entity) in &mut moving_transform_query {
        let Stop {
            collision,
            static_entity,
        } = stop;

        let moving_transform_size = moving_sprite.custom_size.unwrap();
        let static_transform = static_transform_query
            .get_component::<Transform>(*static_entity)
            .unwrap();

        let static_transform_size = static_transform_query
            .get_component::<Sprite>(*static_entity)
            .unwrap()
            .custom_size
            .unwrap();

        match collision {
            Collision::Left => {
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

        commands.entity(entity).remove::<Stop>();
    }
}

// Specifies that the entity can shoot
#[derive(Component)]
pub struct ShootingEvent(pub Entity);

// Spawn a bullet that moves in the direction the transform is facing.
fn shoot(
    mut shooting_event: EventReader<ShootingEvent>,
    mut commands: Commands,
    transform_query: Query<&Transform>,
    bullet_sprite: Res<BulletSprite>,
) {
    for ShootingEvent(entity) in shooting_event.iter() {
        let transform = transform_query
            .get_component::<Transform>(*entity)
            .expect("Everything that shoots needs a transform.");

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..Default::default()
                },
                texture: bullet_sprite.0.clone(),
                // Scale the local y unit vector so that the bullet does not
                // immediately collide with the transform.
                transform: Transform::from_translation(
                    transform.translation + transform.local_y() * 50.0,
                ),
                ..Default::default()
            })
            .insert(Bullet)
            .insert(Movement {
                // 400.0 is the speed of the bullets
                velocity: transform.local_y() * 400.0,
            })
            .insert(ColliderType::Reflect);
    }
}

// This is not physics actually
#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
}

// Does fairly basic linear movement
fn move_transforms(mut query: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.velocity * FIXED_TIMESTEP as f32;
    }
}
