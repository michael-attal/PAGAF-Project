use std::f32::consts::PI;
use std::fmt::Debug;
use crate::tile_loader::TileAssets;
use crate::undo_redo::{Action, UndoRedo};
use crate::wfc::WFCState;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::app_config::DestroyableEntity;
use crate::game::GamePause;
// use bevy_hanabi::prelude::*;
use bevy::prelude::AlphaMode;
use bevy_ghx_proc_gen::proc_gen::generator::model::ModelRotation;
use bevy_ghx_proc_gen::proc_gen::generator::observer::GenerationUpdate;


// #[cfg(not(target_arch = "wasm32"))]
// use crate::particle_fx::{ParticleEffects, spawn_on_place};


//#[cfg(target_arch = "wasm32")]
use crate::particle_fx::spawn_on_place;


#[derive(Resource, Debug, Clone, Copy)]
pub struct TileMapSettings {
    pub width: usize,
    pub height: usize,
}

impl Default for TileMapSettings {
    fn default() -> Self {
        Self { width: 50, height: 50 }
    }
}

#[derive(Component)]
pub struct TileModelTag;

#[derive(Component)]
pub struct PlacementHighlight;

#[derive(Component)]
pub struct ValidationIndicator {
    pub valid: bool,
}

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
    RoadL = 5,
    RoadT = 6,
    RoadX = 7,
    Park = 8
}

/*
#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_effect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    effects: &Res<ParticleEffects>,
    position: Vec3,
) {
    spawn_on_place(commands, asset_server, effects.spawn_handle.clone(), position);
}
*/


//#[cfg(target_arch = "wasm32")]
pub fn spawn_effect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_on_place(commands, meshes, materials, position);
}

impl TileType {
    /// Returns the GLB scene path depending on map size.
    pub fn scene_path_for(self, tile_map_settings: &TileMapSettings) -> Option<String> {
        let use_light = tile_map_settings.width * tile_map_settings.height > 225;
        match self {
            TileType::Empty => None,
            TileType::Road => Some(
                if use_light {
                    "models/newBuilds/roads/2WayRoad.glb#Scene0"
                } else {
                    "models/newBuilds/roads/2WayRoad.glb#Scene0"
                }.to_string()
            ),
            TileType::RoadL => Some(
                if use_light {
                    "models/newBuilds/roads/LTurnRoad.glb#Scene0"
                } else {
                    "models/newBuilds/roads/LTurnRoad.glb#Scene0"
                }.to_string()
            ),
            TileType::RoadT => Some(
                if use_light {
                    "models/newBuilds/roads/TCrossRoad.glb#Scene0"
                } else {
                    "models/newBuilds/roads/TCrossRoad.glb#Scene0"
                }.to_string()
            ),
            TileType::RoadX => Some(
                if use_light {
                    "models/newBuilds/roads/XCrossRoad.glb#Scene0"
                } else {
                    "models/newBuilds/roads/XCrossRoad.glb#Scene0"
                }.to_string()
            ),
            TileType::Residential => Some(
                if use_light {
                    "models/tiles/residential/residential_1.glb#Scene0"
                } else {
                    "models/newBuilds/house/house_1.glb#Scene0"
                }.to_string()
            ),
            TileType::Commercial => Some(
                if use_light {
                    "models/tiles/commercial/commercial_1.glb#Scene0"
                } else {
                    "models/newBuilds/store/store_1.glb#Scene0"
                }.to_string()
            ),
            TileType::Industrial => Some(
                if use_light {
                    "models/tiles/industrial/industrial_1.glb#Scene0"
                } else {
                    "models/newBuilds/factory/factory_1.glb#Scene0"
                }.to_string()
            ),
            TileType::Park => Some(
                if use_light {
                    "models/newBuilds/parc/parcFloorTree.glb#Scene0"
                } else {
                    "models/newBuilds/parc/parcFloorTree.glb#Scene0"
                }.to_string()
            ),
        }
    }

    /// Returns the scale depending on map size.
    pub fn scale_for(self, tile_map_settings: &TileMapSettings) -> Vec3 {
        let use_light = tile_map_settings.width * tile_map_settings.height > 225;
        match self {
            TileType::Residential => if use_light { Vec3::splat(0.1) } else { Vec3::splat(0.5) },
            TileType::Commercial => if use_light { Vec3::splat(0.05) } else { Vec3::splat(0.5) },
            TileType::Industrial => if use_light { Vec3::splat(0.06) } else { Vec3::splat(1.0) },
            TileType::Road => if use_light { Vec3::splat(0.5) } else { Vec3::splat(0.5) },
            TileType::RoadL => if use_light { Vec3::splat(0.5) } else { Vec3::splat(0.5) },
            TileType::RoadT => if use_light { Vec3::splat(0.5) } else { Vec3::splat(0.5) },
            TileType::RoadX => if use_light { Vec3::splat(0.5) } else { Vec3::splat(0.5) },
            TileType::Park => if use_light { Vec3::splat(1.0) } else { Vec3::splat(1.0) },
            TileType::Empty => Vec3::ONE,
        }
    }
}

impl TileType {
    pub const ALL: [TileType; 8] = [
        TileType::Residential,
        TileType::Commercial,
        TileType::Industrial,
        TileType::Road,
        TileType::RoadL,
        TileType::RoadT,
        TileType::RoadX,
        TileType::Park
    ];

    /// Index used in vectors and for file names.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Path of the GLB scene for this tile.
    pub fn scene_path(self) -> Option<String> {
        match self {
            TileType::Empty => None,
            TileType::Road => Some("models/newBuilds/roads/2WayRoad.glb#Scene0".parse().unwrap()),
            TileType::RoadL => Some("models/newBuilds/roads/LTurnRoad.glb#Scene0".parse().unwrap()),
            TileType::RoadT => Some("models/newBuilds/roads/TCrossRoad.glb#Scene0".parse().unwrap()),
            TileType::RoadX => Some("models/newBuilds/roads/XCrossRoad.glb#Scene0".parse().unwrap()),
            TileType::Residential => Some("models/newBuilds/house/house_1.glb#Scene0".parse().unwrap()),
            TileType::Commercial => Some("models/newBuilds/store/store_1.glb#Scene0".parse().unwrap()),
            TileType::Industrial => Some("models/newBuilds/factory/factory_1.glb#Scene0".parse().unwrap()),
            TileType::Park => Some("models/newBuilds/parc/parcFloorTree.glb#Scene0".parse().unwrap())
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

    // To scale cases and models
    pub fn scale(self) -> Vec3 {
        match self {
            TileType::Residential => Vec3::splat(0.5),
            TileType::Commercial => Vec3::splat(0.5),
            TileType::Industrial => Vec3::splat(1.0),
            TileType::Road => Vec3::splat(0.5),
            TileType::RoadL => Vec3::splat(0.5),
            TileType::RoadT => Vec3::splat(0.5),
            TileType::RoadX => Vec3::splat(0.5),
            TileType::Park => Vec3::splat(1.0),
            TileType::Empty => Vec3::ONE,
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

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
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
pub struct GridTile;

// Resource for highlighting materials
#[derive(Resource)]
pub struct HighlightMaterials {
    valid: Handle<StandardMaterial>,
    invalid: Handle<StandardMaterial>,
    preview: Handle<StandardMaterial>,
}

// Setup function to create the grid mesh
pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<TileMapSettings>,
    query: Query<Entity, With<GridTile>>,
) {
    // Remove previous grid tiles before recreating grid
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let grid_size_x = settings.width;
    let grid_size_z = settings.height;
    let tile_size = 1.0;

    let tile_mesh = meshes.add(Plane3d::default().mesh().size(tile_size, tile_size));
    let tile_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        perceptual_roughness: 1.0,
        ..default()
    });

    let highlight_materials = HighlightMaterials {
        valid: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 1.0, 0.0, 0.3),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        invalid: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 0.3),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        preview: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 1.0, 0.3),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(highlight_materials);

    for x in 0..grid_size_x {
        for z in 0..grid_size_z {
            commands.spawn((
                Mesh3d(tile_mesh.clone()),
                MeshMaterial3d(tile_material.clone()),
                Transform::from_xyz(x as f32 * tile_size, 0.0, z as f32 * tile_size),
                GridTile,
            ));
        }
    }

    commands.insert_resource(TileMap::new(grid_size_x, grid_size_z));
}

pub fn place_tile_preview(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    mut selected_tile: ResMut<SelectedTile>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut tile_map: ResMut<TileMap>,
    mut wfc_state: ResMut<WFCState>,
    mut undo_redo: ResMut<UndoRedo>,
    game_pause: Res<GamePause>,
    mut preview: Local<Option<Entity>>,
    mut egui_contexts: EguiContexts,

    // #[cfg(not(target_arch = "wasm32"))]
    // effects: Res<ParticleEffects>,

    asset_server: Res<AssetServer>,

    // #[cfg(target_arch = "wasm32")]
    mut meshes: ResMut<Assets<Mesh>>,

    // #[cfg(target_arch = "wasm32")]
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_map_settings: ResMut<TileMapSettings>,
) {
    if game_pause.paused || egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if selected_tile.0 == TileType::Empty {
        if let Some(entity) = *preview {
            commands.entity(entity).despawn_recursive();
            *preview = None;
        }
        return;
    }

    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera.get_single() else { return };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos).ok() {
            let plane = InfinitePlane3d::new(Vec3::Y);
            if let Some(distance) = ray.intersect_plane(Vec3::ZERO, plane) {
                let intersection = ray.get_point(distance);
                let x = intersection.x.round() as i32;
                let z = intersection.z.round() as i32;

                if x >= 0 && x < tile_map.width as i32 && z >= 0 && z < tile_map.height as i32 {
                    let x = x as usize;
                    let z = z as usize;

                    let can_place = wfc_state.grid.can_place_tile(x, z, selected_tile.0);
                    let tile_handle = tile_assets.tiles[selected_tile.0.index()].clone();

                    if mouse_input.just_pressed(MouseButton::Left) && can_place {

                        /*
                        #[cfg(not(target_arch = "wasm32"))]
                        let placed = place_tile(
                            &mut commands,
                            &mut tile_map,
                            &game_pause,
                            &mut wfc_state,
                            &tile_assets,
                            &selected_tile,
                            &mut undo_redo,
                            x,
                            z,
                            &effects,
                            &asset_server,
                        );
                        */

                        // #[cfg(target_arch = "wasm32")]
                        let placed = place_tile(
                            &mut commands,
                            &mut tile_map,
                            &game_pause,
                            &mut wfc_state,
                            &tile_assets,
                            &selected_tile,
                            &mut undo_redo,
                            x,
                            z,
                            &(),
                            &asset_server,
                            &mut meshes,
                            &mut materials,
                            &mut tile_map_settings,
                        );
                        
                        if placed {
                            if let Some(entity) = *preview {
                                commands.entity(entity).despawn_recursive();
                                *preview = None;
                            }
                        }
                    } else {
                        if let Some(entity) = *preview {
                            commands.entity(entity).despawn_recursive();
                        }
                        *preview = Some(
                            commands
                                .spawn((
                                    SceneRoot(tile_handle),
                                    Transform {
                                        translation: Vec3::new(x as f32, 0.01, z as f32),
                                        scale: selected_tile.0.scale_for(&tile_map_settings),
                                        ..default()
                                    },
                                    TileModelTag,
                                ))
                                .id(),
                        );
                    }
                }
            }
        }
    }
}

/// Places a tile at the given coordinates. Returns true if placement succeeded.
pub fn place_tile(
    commands: &mut Commands,
    tile_map: &mut TileMap,
    game_pause: &GamePause,
    wfc_state: &mut WFCState,
    tile_assets: &TileAssets,
    selected_tile: &SelectedTile,
    undo_redo: &mut UndoRedo,
    x: usize,
    z: usize,

    // #[cfg(not(target_arch = "wasm32"))]
    // effects: &Res<ParticleEffects>,

    // #[cfg(target_arch = "wasm32")]
    effects: &(),

    asset_server: &Res<AssetServer>,

    // #[cfg(target_arch = "wasm32")]
    meshes: &mut ResMut<Assets<Mesh>>,

    // #[cfg(target_arch = "wasm32")]
    materials: &mut ResMut<Assets<StandardMaterial>>,

    tile_map_settings: &mut TileMapSettings,
) -> bool {
    if game_pause.paused
        || selected_tile.0 == TileType::Empty
        || x >= tile_map.width
        || z >= tile_map.height
        || !wfc_state.grid.can_place_tile(x, z, selected_tile.0)
    {
        return false;
    }

    if wfc_state.grid.place_tile(x, z, selected_tile.0) {
        return update_map(
            commands,
            tile_map,
            wfc_state,
            tile_assets,
            undo_redo,
            meshes,
            materials,
            asset_server,
            tile_map_settings
        )
    }
    
    false
}

pub fn update_map(
    commands: &mut Commands,
    tile_map: &mut TileMap,
    wfc_state: &mut WFCState,
    tile_assets: &TileAssets,
    undo_redo: &mut UndoRedo,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    tile_map_settings: &mut TileMapSettings,
) -> bool
{
    let updates = wfc_state.grid.observer.dequeue_all();
    for i in 0..updates.len()
    {
        let update = updates[i];

        match update {
            GenerationUpdate::Generated(node) => {
                let coords = wfc_state.grid.pos_from_index(node.node_index);
                let tile_type = wfc_state.grid.tiletype_from_id(node.model_instance.model_index);
                let rotation = match node.model_instance.rotation {
                    ModelRotation::Rot0 => 0.0,
                    ModelRotation::Rot90 => -90.0 * PI / 180.0,
                    ModelRotation::Rot180 => 180.0 * PI / 180.0,
                    ModelRotation::Rot270 => -270.0 * PI / 180.0
                };

                let scene_entity = commands
                    .spawn((
                        DestroyableEntity,
                        SceneRoot(tile_assets.tiles[tile_type.index()].clone()),
                        Transform {
                            translation: Vec3::new(coords.x as f32, 0.01, coords.y as f32),
                            scale: tile_type.scale_for(tile_map_settings),
                            rotation: Quat::from_rotation_y(rotation),
                            ..default()
                        },
                        TileModelTag,
                    ))
                    .id();

                let pos = Vec3::new(coords.x as f32, 0.5, coords.y as f32);

                // #[cfg(not(target_arch = "wasm32"))]
                // spawn_effect(commands, asset_server, effects, pos);

                // #[cfg(target_arch = "wasm32")]
                spawn_effect(commands, asset_server, meshes, materials, pos);

                tile_map.tiles[coords.y as usize][coords.x as usize].tile_type = tile_type;
                tile_map.entities[coords.y as usize][coords.x as usize] = Some(scene_entity);
                undo_redo.add_action(Action::PlaceTile(coords.x as usize, coords.y as usize, tile_type));
            }

            GenerationUpdate::Failed(fail) => {
                println!("On s'est chié dessus! {}", fail);
                return false
            }

            GenerationUpdate::Reinitializing(seed) => {
                println!("All is null");
                return false
            }
        }
    }

    true
}

pub fn update_placement_highlights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    wfc_state: Res<WFCState>,
    selected_tile: Res<SelectedTile>,
    highlight_materials: Res<HighlightMaterials>,
    query: Query<Entity, With<PlacementHighlight>>,
) {
    // Removes old highlights
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    if selected_tile.0 == TileType::Empty {
        return;
    }

    let tile_mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));

    // For each grid cell
    for y in 0..wfc_state.grid.height {
        for x in 0..wfc_state.grid.width {

            if !wfc_state.grid.collapsed(x, y) {
                let material = if wfc_state.grid.can_place_tile(x, y, selected_tile.0) {
                    highlight_materials.valid.clone()
                } else {
                    highlight_materials.invalid.clone()
                };

                commands.spawn((
                    DestroyableEntity,
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, 0.02, y as f32),
                    PlacementHighlight,
                ));
            }
        }
    }
}

// Resource for currently selected tile type (to be set via UI)
#[derive(Resource)]
pub struct SelectedTile(pub TileType);
