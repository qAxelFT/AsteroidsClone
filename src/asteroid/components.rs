use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid {
    pub speed: f32,
    pub direction: Vec2,
    pub size: f32,
}