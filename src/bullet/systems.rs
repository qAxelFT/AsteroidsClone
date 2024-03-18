use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::asteroid::components::*;
use crate::player::components::*;

pub fn shoot_bullet(
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let _window = window_query.get_single().unwrap();
    let player_transform = player_query.single();

    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    0.0,
                ),
                texture: asset_server.load("bullet.png"),
                ..default()
            },
            Bullet {
                speed: 200.0,
                size: 8.0,
            },
        ));
    }
}

pub fn bullet_movement(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_query {
        let direction = Vec3::new(0.0, 1.0, 0.0);

        transform.translation += direction * bullet.speed * time.delta_seconds();
    }
}

pub fn despawn_bullet(
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (bullet_entity, transform, bullet) in &bullet_query {
        let half_size = bullet.size / 2.0;

        let y_max = window.height() - half_size;

        if transform.translation.y > y_max {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn bullet_asteroid_collision(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
) {
    for (bullet_entity, bullet_transform, bullet) in &bullet_query {
        for (asteroid_entity, asteroid_transform, asteroid) in &asteroid_query {
            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);
            let bullet_radius = bullet.size / 2.0;
            let asteroid_radius = asteroid.size / 2.0;

            if distance < bullet_radius + asteroid_radius {
                commands.entity(asteroid_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}