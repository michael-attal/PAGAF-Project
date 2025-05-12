use bevy::prelude::*;

#[derive(Resource)]
pub struct GamePause {
    pub paused: bool,
}

impl Default for GamePause {
    fn default() -> Self {
        Self { paused: false }
    }
}

pub fn setup_game(mut commands: Commands) {
    // Camera setup
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Lighting setup
    commands.spawn((
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
    time: Res<Time>,
) {
    let speed = 10.0;

    if let Ok(mut transform) = query.single_mut() {
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= speed * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.z += speed * time.delta_secs();
        }
    }
}
