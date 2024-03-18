use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot_bullet,
            bullet_movement,
            despawn_bullet,
            bullet_asteroid_collision,));
    }
}