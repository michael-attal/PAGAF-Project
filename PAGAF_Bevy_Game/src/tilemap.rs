use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: IVec2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Residential,
    Commercial,
    Industrial,
    Road,
    Park,
}

#[derive(Resource)]
pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Default for TileMap {
    fn default() -> Self {
        let width = 50;
        let height = 50;
        let mut tiles = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(Tile {
                    tile_type: TileType::Empty,
                    position: IVec2::new(x as i32, y as i32),
                });
            }
            tiles.push(row);
        }

        Self {
            tiles,
            width,
            height,
        }
    }
}

pub fn setup_tilemap(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(TileMap::default());

    // TODO: Create mesh for the grid
}

// TODO: Create the wave function collapse algorithm

// TODO: Create building generation algorithm with the wave function collapse algorithm and the default tile map setup mesh