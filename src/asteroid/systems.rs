use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;

pub fn tick_enemy_spawner(mut enemy_timer: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}

pub fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_spawn: Res<EnemyTimer>,
) {
    if enemy_spawn.timer.finished() {
        let window = window_query.get_single().unwrap();

        for _i in 0..5 {
            let rand_speed = rand::thread_rng().gen_range(40.0..101.0);
            let rand_texture = rand::thread_rng().gen_range(0..2);
            let rand_x = random::<f32>() * window.width();
            let rand_y = random::<f32>() * window.width();
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load(format!("asteroid_{}.png", rand_texture)),
                    ..default()
                },
                Asteroid {
                    speed: rand_speed,
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                    size: 32.0,
                },
            ));
        }
    }
}

pub fn asteroid_movement(
    mut enemy_query: Query<(&mut Transform, &Asteroid), With<Asteroid>>,
    time: Res<Time>,
) {
    for (mut enemy_transform, enemy) in &mut enemy_query {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        enemy_transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn update_asteroid(
    mut enemy_query: Query<(&Transform, &mut Asteroid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (enemy_tranform, mut enemy) in &mut enemy_query {
        let half_size = enemy.size / 2.0;

        let x_min = 0.0 + half_size;
        let x_max = window.width() - half_size;
        let y_min = 0.0 + half_size;
        let y_max = window.height() - half_size;

        let translation = enemy_tranform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}
