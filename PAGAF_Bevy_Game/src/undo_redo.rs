use crate::tilemap::{TileMap, TileType};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UndoRedo {
    pub history: Vec<Action>,
    pub redo_stack: Vec<Action>,
}

#[derive(Clone)]
pub enum Action {
    PlaceTile(usize, usize, TileType),
    RemoveTile(usize, usize, TileType),
}

impl UndoRedo {
    // Add an action and clear redo stack
    pub fn add_action(&mut self, action: Action) {
        self.history.push(action);
        self.redo_stack.clear();
    }

    // Undo the last action
    pub fn undo(&mut self, tilemap: &mut TileMap) {
        if let Some(action) = self.history.pop() {
            match &action {
                Action::PlaceTile(x, y, _) => {
                    self.redo_stack.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = TileType::Empty;
                }
                Action::RemoveTile(x, y, tile_type) => {
                    self.redo_stack.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = *tile_type;
                }
            }
        }
    }

    // Redo the last undone action
    pub fn redo(&mut self, tilemap: &mut TileMap) {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                Action::PlaceTile(x, y, tile_type) => {
                    self.history.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = *tile_type;
                }
                Action::RemoveTile(x, y, _) => {
                    self.history.push(action.clone());
                    tilemap.tiles[*y][*x].tile_type = TileType::Empty;
                }
            }
        }
    }
}

// UI system for Undo/Redo buttons
pub fn undo_redo_ui(
    mut contexts: bevy_egui::EguiContexts,
    mut undo_redo: ResMut<UndoRedo>,
    mut tilemap: ResMut<TileMap>,
) {
    bevy_egui::egui::Window::new("Undo/Redo")
        .anchor(
            bevy_egui::egui::Align2::LEFT_TOP,
            bevy_egui::egui::vec2(10.0, 10.0),
        )
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Undo").clicked() {
                    undo_redo.undo(&mut tilemap);
                }
                if ui.button("Redo").clicked() {
                    undo_redo.redo(&mut tilemap);
                }
            });
        });
}
