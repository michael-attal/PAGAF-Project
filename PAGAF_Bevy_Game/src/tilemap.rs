use crate::tile_loader::TileAssets;
use crate::undo_redo::{Action, UndoRedo};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

// Tile component storing type and position
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: IVec2,
}

// Enum representing different types of tiles
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Empty = 0,
    Residential = 1,
    Commercial = 2,
    Industrial = 3,
    Road = 4,
    Park = 5,
}

impl TileType {
    pub const ALL: [TileType; 5] = [
        TileType::Residential,
        TileType::Commercial,
        TileType::Industrial,
        TileType::Road,
        TileType::Park,
    ];

    /// Index used in vectors and for file names.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Path of the GLB scene for this tile.
    pub fn scene_path(self) -> Option<String> {
        match self {
            TileType::Empty => None,
            _ => Some(format!(
                "models/tiles/tile_{}/tile.glb#Scene0",
                //self.index()
                1
            )),
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(TileType::Empty),
            1 => Some(TileType::Residential),
            2 => Some(TileType::Commercial),
            3 => Some(TileType::Industrial),
            4 => Some(TileType::Road),
            5 => Some(TileType::Park),
            _ => None,
        }
    }
}

// Resource to store the map data

#[derive(Resource)]
pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub entities: Vec<Vec<Option<Entity>>>, // Track spawned tile entities
}

impl Default for TileMap {
    fn default() -> Self {
        let width = 50;
        let height = 50;
        let mut tiles = Vec::with_capacity(height);
        let mut entities = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            let mut entity_row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(Tile {
                    tile_type: TileType::Empty,
                    position: IVec2::new(x as i32, y as i32),
                });
                entity_row.push(None);
            }
            tiles.push(row);
            entities.push(entity_row);
        }

        Self {
            tiles,
            width,
            height,
            entities,
        }
    }
}

// Marker component for grid tiles
#[derive(Component)]
struct GridTile;

// Setup function to create the grid mesh
pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_size = 50;
    let tile_size = 1.0;

    let tile_mesh = meshes.add(Plane3d::default().mesh().size(tile_size, tile_size));
    let tile_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        perceptual_roughness: 1.0,
        ..default()
    });

    for x in 0..grid_size {
        for z in 0..grid_size {
            commands.spawn((
                Mesh3d(tile_mesh.clone()),
                MeshMaterial3d(tile_material.clone()),
                Transform::from_xyz(x as f32 * tile_size, 0.0, z as f32 * tile_size),
                GridTile,
            ));
        }
    }

    commands.insert_resource(TileMap::default());
}

pub fn place_tile_preview(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    mut selected_tile: ResMut<SelectedTile>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut tile_map: ResMut<TileMap>,
    mut undo_redo: ResMut<UndoRedo>,
    mut preview: Local<Option<Entity>>,
    mut egui_contexts: EguiContexts,
) {
    // Block input if pointer is over UI
    if egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    // Do nothing if no tile selected
    if selected_tile.0 == TileType::Empty {
        if let Some(entity) = *preview {
            commands.entity(entity).despawn_recursive();
            *preview = None;
        }
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos).ok() {
            let plane = InfinitePlane3d::new(Vec3::Y);
            if let Some(distance) = ray.intersect_plane(Vec3::ZERO, plane) {
                let intersection = ray.get_point(distance);
                let x = intersection.x.round() as i32;
                let z = intersection.z.round() as i32;

                if x >= 0 && x < tile_map.width as i32 && z >= 0 && z < tile_map.height as i32 {
                    let tile_handle = tile_assets.tiles[selected_tile.0.index()].clone();

                    if mouse_input.just_pressed(MouseButton::Left) {
                        // Spawn tile in the world
                        let entity = commands
                            .spawn((
                                SceneRoot(tile_handle.clone()),
                                Transform::from_xyz(x as f32, 0.0, z as f32),
                            ))
                            .id();

                        tile_map.tiles[z as usize][x as usize].tile_type = selected_tile.0;
                        tile_map.entities[z as usize][x as usize] = Some(entity);
                        undo_redo.add_action(Action::PlaceTile(
                            x as usize,
                            z as usize,
                            selected_tile.0,
                        ));

                        // Remove preview
                        if let Some(entity) = *preview {
                            commands.entity(entity).despawn_recursive();
                            *preview = None;
                        }

                        // Deselect after placing
                        selected_tile.0 = TileType::Empty;
                    } else {
                        // Update preview
                        if let Some(entity) = *preview {
                            commands.entity(entity).despawn_recursive();
                        }
                        *preview = Some(
                            commands
                                .spawn((
                                    SceneRoot(tile_handle),
                                    Transform::from_xyz(x as f32, 0.01, z as f32),
                                ))
                                .id(),
                        );
                    }
                }
            }
        }
    }
}

// System to place a tile at a specific grid position
pub fn place_tile(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    selected_tile: Res<SelectedTile>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut undo_redo: ResMut<UndoRedo>,
    mut tile_map: ResMut<TileMap>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos).ok() {
                let plane = InfinitePlane3d::new(Vec3::Y);
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, plane) {
                    let intersection = ray.get_point(distance);
                    let x = intersection.x.round() as i32;
                    let z = intersection.z.round() as i32;

                    if x >= 0 && x < tile_map.width as i32 && z >= 0 && z < tile_map.height as i32 {
                        let tile_handle = tile_assets.tiles[selected_tile.0.index()].clone();

                        // Spawn the tile scene at grid position
                        commands.spawn((
                            SceneRoot(tile_handle),
                            Transform::from_xyz(x as f32, 0.0, z as f32),
                        ));

                        // Add tile to TileMap resource
                        tile_map.tiles[z as usize][x as usize].tile_type = selected_tile.0;

                        // Add action to UndoRedo
                        undo_redo.add_action(Action::PlaceTile(
                            x as usize,
                            z as usize,
                            selected_tile.0,
                        ));
                    }
                }
            }
        }
    }
}

// Resource for currently selected tile type (to be set via UI)
#[derive(Resource)]
pub struct SelectedTile(pub TileType);
