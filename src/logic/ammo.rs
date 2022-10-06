use crate::logic::enemy::EnemyDeathEvent;
use crate::logic::player::{Player, STARTING_AMMO};
use bevy::prelude::*;

pub struct AmmoPlugin;

pub const BULLET_WIDTH: f32 = 10.0;
pub const BULLET_HEIGHT: f32 = 30.0;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ammo)
            .add_startup_system(show_ammo_ui)
            .add_system(update_ammo_ui);
    }
}

#[derive(Component)]
pub struct Ammo;

// Ammo spawns from dead enemies
fn spawn_ammo(mut enemy_death_events: EventReader<EnemyDeathEvent>, mut commands: Commands) {
    for enemy_death_event in enemy_death_events.iter() {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_WIDTH, BULLET_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_translation(enemy_death_event.death_position),
                ..Default::default()
            })
            .insert(Ammo);
    }
}

#[derive(Component)]
pub struct AmmoUI;

// TODO make the ammo UI look nicer
// For now ammo is just a number in the bottom right
fn show_ammo_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let score_text: Text = Text::from_sections([
        TextSection::new(
            "Ammo: ",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        TextSection::new(
            STARTING_AMMO.to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::BLUE,
            },
        ),
    ]);

    commands
        .spawn()
        .insert_bundle(TextBundle {
            text: score_text,
            style: Style {
                // Flex end is making the node go to the top of the screen
                // since the origin is in the bottom left.
                align_self: AlignSelf::Baseline,
                // Move the node a little over
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AmmoUI);
}

// Update the ammo ui whenever the player's ammo changes
fn update_ammo_ui(
    mut ammo_ui_query: Query<&mut Text, With<AmmoUI>>,
    player_query: Query<&Player, Changed<Player>>,
) {
    let mut ammo_ui = ammo_ui_query
        .get_single_mut()
        .expect("Could not find the ammo ui");

    // NOTE
    // This requires a loop because most of the time no player component is changing
    for player in &player_query {
        ammo_ui.sections[1].value = player.ammo.to_string();
    }
}
