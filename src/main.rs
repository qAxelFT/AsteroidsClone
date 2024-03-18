use bevy::{prelude::*, window::PrimaryWindow};

pub mod player;
pub mod asteroid;
pub mod bullet;

use crate::player::*;
use crate::asteroid::*;
use crate::bullet::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), PlayerPlugin, AsteroidPlugin, BulletPlugin))
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
    ));
}