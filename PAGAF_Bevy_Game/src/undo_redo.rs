use bevy::prelude::*;

use crate::tile_loader::TileAssets;
use crate::tilemap::{TileMap, TileType};

#[derive(Debug, Clone)]
pub enum Action {
    PlaceTile(usize, usize, TileType),
    RemoveTile(usize, usize, TileType),
}

#[derive(Resource, Default)]
pub struct UndoRedo {
    pub history: Vec<Action>,
    pub redo_stack: Vec<Action>,
}

impl UndoRedo {
    pub fn add_action(&mut self, action: Action) {
        self.history.push(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, tilemap: &mut TileMap, commands: &mut Commands) {
        if let Some(action) = self.history.pop() {
            match &action {
                Action::PlaceTile(x, y, _) => {
                    self.redo_stack.push(action.clone());

                    // Remove visual tile if present
                    if let Some(entity) = tilemap.entities[*y][*x].take() {
                        commands.entity(entity).despawn_recursive();
                    }

                    tilemap.tiles[*y][*x].tile_type = TileType::Empty;
                }
                Action::RemoveTile(x, y, old_type) => {
                    self.redo_stack.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = *old_type;
                }
            }
        }
    }

    pub fn redo(
        &mut self,
        tilemap: &mut TileMap,
        commands: &mut Commands,
        tile_assets: &Res<TileAssets>,
    ) {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                Action::PlaceTile(x, y, tile_type) => {
                    self.history.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = *tile_type;

                    let handle = tile_assets.tiles[tile_type.index()].clone();
                    let entity = commands
                        .spawn((
                            SceneRoot(handle),
                            Transform::from_xyz(*x as f32, 0.0, *y as f32),
                        ))
                        .id();
                    tilemap.entities[*y][*x] = Some(entity);
                }
                Action::RemoveTile(x, y, _) => {
                    self.history.push(action.clone());

                    if let Some(entity) = tilemap.entities[*y][*x].take() {
                        commands.entity(entity).despawn_recursive();
                    }

                    tilemap.tiles[*y][*x].tile_type = TileType::Empty;
                }
            }
        }
    }
}
