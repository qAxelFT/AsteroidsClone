use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::asteroid::components::Asteroid;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player {
            speed: 100.0,
            size: 32.0,
        },
    ));
}

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player_query {
        let mut direction: Vec3 = Vec3::ZERO;

        if input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * player.speed * time.delta_seconds();
    }
}

pub fn player_asteroid_collision(
    player_query: Query<(Entity, &Transform, &Player)>,
    asteroid_query: Query<(&Transform, &Asteroid)>,
    mut commands: Commands,
) {
    for (player_entity, player_transform, player) in &player_query {
        for (asteroid_transform, asteroid) in &asteroid_query {
            let distance = player_transform
                .translation
                .distance(asteroid_transform.translation);
            let player_radius = player.size / 2.0;
            let asteroid_radius = asteroid.size / 2.0;

            if distance < player_radius + asteroid_radius {
                commands.entity(player_entity).despawn();
            }
        }
    }
}
