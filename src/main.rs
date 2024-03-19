use bevy::prelude::*;

pub mod asteroid;
pub mod bullet;
pub mod player;
pub mod score;
pub mod systems;

use crate::asteroid::*;
use crate::bullet::*;
use crate::player::*;
use crate::score::*;
use crate::systems::spawn_camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            AsteroidPlugin,
            BulletPlugin,
            ScorePlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
