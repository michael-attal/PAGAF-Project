use bevy::prelude::*;
use bevy::math::primitives::Cuboid;

#[derive(Resource)]
pub struct GamePause {
    pub paused: bool,
}

impl Default for GamePause {
    fn default() -> Self {
        Self { paused: false }
    }
}


#[derive(Component)]
pub struct RotatingCube;

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    commands.spawn((
        PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
    ));

    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#ffd891").unwrap().into(),
            unlit: false,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingCube,
    ));
}

pub fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotate(Quat::from_rotation_y(time.delta().as_secs_f32()));
    }
}