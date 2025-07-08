use bevy::prelude::*;
use crate::app_config::DestroyableEntity;
use crate::tilemap::{SelectedTile, TileType};
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use std::ops::Range;

#[derive(Resource)]
pub struct GamePause {
    pub paused: bool,
}

impl Default for GamePause {
    fn default() -> Self {
        Self { paused: false }
    }
}

pub fn setup_game(mut commands: Commands, entity_query: Query<Entity, With<DestroyableEntity>>, mut selected_tile: ResMut<SelectedTile>) {

    selected_tile.0 = TileType::Empty;

    for e in entity_query.iter() {
        commands.entity(e).despawn();
    }

    // Camera setup - Now in skybox
    /*
    commands.spawn((
        DestroyableEntity,
        Camera3d::default(),
        Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    */

    // Lighting setup
    commands.spawn((
        DestroyableEntity,
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
pub fn camera_movement(
    mut query: Query<&mut Transform, With<Camera3d>>,
    input: Res<ButtonInput<KeyCode>>,
    game_pause: Res<GamePause>,
    time: Res<Time>,
) {

    if game_pause.paused{
        return;
    }

    let speed = 10.0;
    let rotation_speed = 1.0;

    if let Ok(mut transform) = query.single_mut() {
        let mut movement:Vec3 = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            let mut direction:Vec3 = transform.right().as_vec3();
            direction.y = 0.0;
            movement -= direction;
        }
        if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            let mut direction:Vec3 = transform.right().as_vec3();
            direction.y = 0.0;
            movement += direction;
        }
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            let mut direction:Vec3 = transform.forward().as_vec3();
            direction.y = 0.0;
            movement += direction;
        }
        if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
            let mut direction:Vec3 = transform.forward().as_vec3();
            direction.y = 0.0;
            movement -= direction;
        }

        transform.translation += movement * speed * time.delta_secs();

        if input.pressed(KeyCode::KeyQ) {
            transform.rotate_y(rotation_speed * time.delta_secs());
        }
        if input.pressed(KeyCode::KeyE) {
            transform.rotate_y(-rotation_speed * time.delta_secs());
        }
    }
}

#[derive(Resource)]
pub struct CameraZoomSettings {
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
}

pub fn camera_zoom_system(
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    zoom_settings: Res<CameraZoomSettings>,
) {
    if mouse_scroll.delta.y.abs() > 0.01 {
        if let Ok(mut transform) = query.get_single_mut() {
            let dir = (transform.translation - Vec3::ZERO).normalize();
            let mut distance = (transform.translation - Vec3::ZERO).length();
            distance = (distance - mouse_scroll.delta.y * zoom_settings.zoom_speed)
                .clamp(zoom_settings.zoom_range.start, zoom_settings.zoom_range.end);
            transform.translation = dir * distance;
        }
    }
}