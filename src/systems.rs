use bevy::{prelude::*, window::PrimaryWindow};
use bevy_parallax::*;

pub fn spawn_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let window = window_query.get_single().unwrap();
    let camera = commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .insert(ParallaxCameraComponent::default())
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![LayerData {
            speed: LayerSpeed::Bidirectional(0.9, 0.9),
            path: "back.png".to_string(),
            tile_size: Vec2::new(800.0, 800.0),
            cols: 1,
            rows: 1,
            scale: Vec2::splat(1.0),
            z: -1.0,
            ..default()
        }],
        camera,
    });
}

/*pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
) {
    let (camera, camera_transform) = camera_query.get_single_mut().unwrap();
    let speed = 9.0;
    let mut direction = Vec2::ZERO;
    direction += Vec2::new(0.0, 1.0);
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: direction.normalize_or_zero() * speed,
        camera: camera,
    });
}*/
