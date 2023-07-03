#![feature(slice_group_by)]

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

mod logic;

// TODO
// Im gonna rewrite the physics stuff using events, its probs not that much work tho
// I think I want to learn more about how ECS's work tho. So Im gonna do that then come back to this.
// I know this is taking forever, but Im doing this for fun and because I like learning.
// One day ill start producing games I promise lol

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Close the window to return to the main function".into(),
                resolution: WindowResolution::new(WINDOWWIDTH, WINDOWHEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(load_sprite_assets)
        .add_system(flashing)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::walls::WallsPlugin)
        .add_plugin(logic::bullet::BulletPlugin)
        .add_plugin(logic::enemy::EnemyPlugin)
        .add_plugin(logic::score::ScorePlugin)
        .add_plugin(logic::ammo::AmmoPlugin)
        .add_plugin(logic::physics::PhysicsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

// Enemies spawning in should flash
#[derive(Component)]
pub struct Flashing {
    flashed: bool,
    // The timer refers to how long it takes between flashes
    timer: Timer,
}

fn flashing(mut query: Query<(&mut Sprite, &mut Flashing)>, time: Res<Time>) {
    for (mut sprite, mut flashing) in &mut query {
        if flashing.timer.tick(time.delta()).finished() {
            if !flashing.flashed {
                sprite.color = Color::BLACK;
            } else {
                sprite.color = Color::CRIMSON;
            }
            flashing.flashed = !flashing.flashed;
        }
    }
}

#[derive(Resource)]
pub struct EnemySprite(Handle<Image>);

#[derive(Resource)]
pub struct BulletSprite(Handle<Image>);

#[derive(Resource)]
pub struct TurretSprite(Handle<Image>);

#[derive(Resource)]
pub struct AmmoSprite(Handle<Image>);

// Store sprite assets that I will be accessing over and over at startup.
fn load_sprite_assets(mut commands: Commands, server: Res<AssetServer>) {
    let enemy_handle: Handle<Image> = server.load("sprites/basic-enemy.png");
    let bullet_handle: Handle<Image> = server.load("sprites/bullet.png");
    let turret_handle: Handle<Image> = server.load("sprites/turret.png");
    let ammo_handle: Handle<Image> = server.load("sprites/ammo.png");

    commands.insert_resource(EnemySprite(enemy_handle));
    commands.insert_resource(BulletSprite(bullet_handle));
    commands.insert_resource(TurretSprite(turret_handle));
    commands.insert_resource(AmmoSprite(ammo_handle));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
