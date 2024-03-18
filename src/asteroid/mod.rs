use bevy::prelude::*;

pub mod resources;
pub mod components;
mod systems;

use resources::*;
use systems::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyTimer>().add_systems(Update, (spawn_asteroids,
            asteroid_movement,
            update_asteroid,
            tick_enemy_spawner,));
    }
}