use bevy::prelude::*;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

mod logic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOWWIDTH,
                height: WINDOWHEIGHT,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(load_sprite_assets)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::walls::WallsPlugin)
        .add_plugin(logic::bullet::BulletPlugin)
        .add_plugin(logic::enemy::EnemyPlugin)
        .add_plugin(logic::score::ScorePlugin)
        .add_plugin(logic::ammo::AmmoPlugin)
        .add_plugin(logic::physics::PhysicsPlugin)
        .run();
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
