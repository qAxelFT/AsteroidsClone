use bevy::prelude::*;

use super::components::*;
use super::resources::*;

pub fn setup_scoreboard(mut commands: Commands) {
    commands.spawn((
        ScoreboardUI,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::hex("#FFFFFF").unwrap(),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.0,
                color: Color::hex("#FFFFFF").unwrap(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.value.to_string();
}
