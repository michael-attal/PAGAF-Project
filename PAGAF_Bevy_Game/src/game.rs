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
