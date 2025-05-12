use bevy::{gltf::Gltf, prelude::*};

// Resource storing handles to loaded tiles
#[derive(Resource)]
pub struct TileAssets {
    pub tiles: Vec<Handle<Scene>>,
}

// System to load GLTF tiles at startup
pub fn load_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tiles = Vec::new();

    // Loading all GLTF tiles
    for i in 1..=5 {
        // let path = format!("models/tiles/tile_{}/tile.glb#Scene0", i);
        let path = format!("models/tiles/tile_1/tile.glb#Scene0");
        let handle: Handle<Scene> = asset_server.load(path);
        tiles.push(handle);
    }

    commands.insert_resource(TileAssets { tiles });
}
