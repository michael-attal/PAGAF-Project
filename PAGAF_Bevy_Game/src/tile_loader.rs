use bevy::{gltf::Gltf, prelude::*};
use crate::tilemap::TileType;

/// Resource holding the Scene handles for each tile.
/// Index 0 is `Handle::default()` for `TileType::Empty`.
#[derive(Resource)]
pub struct TileAssets {
    pub tiles: Vec<Handle<Scene>>,
}

pub fn load_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Allocate once with the correct length (= highest enum value + 1)
    let mut tiles = vec![Handle::<Scene>::default(); TileType::Park.index() + 1];

    for tile_type in TileType::ALL {
        if let Some(path) = tile_type.scene_path() {
            let handle: Handle<Scene> = asset_server.load(path);
            tiles[tile_type.index()] = handle;
        }
    }

    commands.insert_resource(TileAssets { tiles });
}
