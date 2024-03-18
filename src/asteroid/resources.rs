use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyTimer {
    pub timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> EnemyTimer {
        EnemyTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}
