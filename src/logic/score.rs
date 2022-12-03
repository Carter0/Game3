use crate::logic::enemy::EnemyDeathEvent;
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_score_ui)
            .add_system(update_score_text);
    }
}

#[derive(Component)]
struct Score {
    value: u32,
}

// NOTE for UI, the origin is in the bottom left corner.
// The score UI is in the top left of the screen
fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let score_text: Text = Text::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        TextSection::new(
            0.to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            },
        ),
    ]);

    commands
        .spawn(TextBundle {
            text: score_text,
            style: Style {
                // Flex end is making the node go to the top of the screen
                // since the origin is in the bottom left.
                align_self: AlignSelf::FlexEnd,
                // Move the node a little over
                position: UiRect {
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score { value: 0 });
}

// Every time an enemy dies the score gets updated by one
fn update_score_text(
    mut enemy_death_events: EventReader<EnemyDeathEvent>,
    mut score_query: Query<(&mut Text, &mut Score)>,
) {
    for _enemy_death_event in enemy_death_events.iter() {
        let (mut score_text, mut score_component) = score_query
            .get_single_mut()
            .expect("Could not find a single score component");

        score_component.value += 1;
        score_text.sections[1].value = score_component.value.to_string();
    }
}
