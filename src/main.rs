use bevy::prelude::*;
use bevy_parallax::*;

pub mod asteroid;
pub mod bullet;
pub mod main_menu;
pub mod player;
pub mod score;
pub mod systems;

use crate::asteroid::*;
use crate::bullet::*;
use crate::player::*;
use crate::score::*;
use crate::systems::spawn_camera;

fn main() {
    let primary_window = Window {
        title: "Cyberpunk".to_string(),
        resolution: (720.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            AsteroidPlugin,
            BulletPlugin,
            ScorePlugin,
            ParallaxPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
