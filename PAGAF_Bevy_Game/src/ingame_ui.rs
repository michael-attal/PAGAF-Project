use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::app_config::{GameSettings, GameState};
use crate::game::{GamePause};

#[derive(Component)]
pub struct TilePanel;

#[derive(Component)]
pub struct GameMenu;

#[derive(Resource)]
pub struct AvailableTiles {
    tiles: Vec<TileType>,
}

#[derive(Clone)]
pub enum TileType {
    Residential,
    Commercial,
    Industrial,
    Road,
    Park,
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
    mut settings: ResMut<GameSettings>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut pause: ResMut<GamePause>,
) {
    // Top right menu
    egui::Window::new("Menu")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .collapsible(true)
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            if ui.button(if pause.paused { "Resume" } else { "Pause" }).clicked() {
                pause.paused = !pause.paused;
            }

            if ui.button("Settings").clicked() {
                next_state.set(GameState::Settings);
            }

            if ui.button("Back to menu").clicked() {
                next_state.set(GameState::MainMenu);
            }

            ui.add_space(20.0);

            if ui.button("Quit game").clicked() {
                exit.write(bevy::app::AppExit::Success);
            }
        });
}

pub fn tile_panel(
    mut contexts: EguiContexts,
    tiles: Res<AvailableTiles>,
) {
    // Bottom panel (Warcraft 3 like)
    egui::Window::new("Building Panel")
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                for tile in &tiles.tiles {
                    let button_text = match tile {
                        TileType::Residential => "🏠",
                        TileType::Commercial => "🏢",
                        TileType::Industrial => "🏭",
                        TileType::Road => "🛣️",
                        TileType::Park => "🌳",
                    };

                    if ui.button(button_text).clicked() {
                        // TODO: Add building
                    }
                }
            });
        });
}