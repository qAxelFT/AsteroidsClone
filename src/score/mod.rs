use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup_scoreboard)
            .add_systems(Update, update_scoreboard);
    }
}
