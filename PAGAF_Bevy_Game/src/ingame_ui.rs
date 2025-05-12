use crate::app_config::{GameSettings, GameState};
use crate::game::GamePause;
use crate::tilemap::{SelectedTile, TileMap, TileType};
use crate::undo_redo::UndoRedo;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct AvailableTiles {
    pub tiles: Vec<TileType>,
}

impl Default for AvailableTiles {
    fn default() -> Self {
        Self {
            tiles: vec![
                TileType::Residential,
                TileType::Commercial,
                TileType::Industrial,
                TileType::Road,
                TileType::Park,
            ],
        }
    }
}

pub fn game_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut pause: ResMut<GamePause>,
) {
    egui::Window::new("Menu")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            if ui
                .button(if pause.paused { "Resume" } else { "Pause" })
                .clicked()
            {
                pause.paused = !pause.paused;
            }
            if ui.button("Settings").clicked() {
                next_state.set(GameState::Settings);
            }
            if ui.button("Main menu").clicked() {
                next_state.set(GameState::MainMenu);
            }
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });
}

pub fn tile_panel(
    mut contexts: EguiContexts,
    tiles: Res<AvailableTiles>,
    mut selected_tile: ResMut<SelectedTile>,
    mut undo_redo: ResMut<UndoRedo>,
    mut tilemap: ResMut<TileMap>,
) {
    egui::Window::new("Building Panel")
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -10.0))
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                for tile in &tiles.tiles {
                    if ui
                        .selectable_label(*tile == selected_tile.0, format!("{:?}", tile))
                        .clicked()
                    {
                        selected_tile.0 = *tile;
                    }
                }
            });

            ui.separator();

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
