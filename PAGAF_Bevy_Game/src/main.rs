mod app_config;
mod game;
mod ingame_ui;
mod tile_loader;
mod tilemap;
mod ui;
mod undo_redo;
mod wfc;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use app_config::{GameSettings, GameState};
use game::GamePause;
use ingame_ui::AvailableTiles;
use tile_loader::load_tiles;
use tilemap::{SelectedTile, TileType, place_tile, setup_grid};
use undo_redo::{UndoRedo, undo_redo_ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .insert_resource(GameSettings::default())
        .insert_resource(GamePause::default())
        .insert_resource(AvailableTiles::default())
        .insert_resource(SelectedTile(TileType::Residential))
        .insert_resource(UndoRedo::default())
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (app_config::play_background_music, load_tiles, setup_grid),
        )
        .add_systems(OnEnter(GameState::InGame), game::setup_game)
        .add_systems(
            Update,
            (
                // UI Menus
                ui::main_menu.run_if(in_state(GameState::MainMenu)),
                ui::settings_menu.run_if(in_state(GameState::Settings)),
                ui::load_game_menu.run_if(in_state(GameState::LoadGame)),
                // In-game systems
                game::camera_movement.run_if(in_state(GameState::InGame)),
                ingame_ui::game_menu.run_if(in_state(GameState::InGame)),
                ingame_ui::tile_panel.run_if(in_state(GameState::InGame)),
                tilemap::place_tile.run_if(in_state(GameState::InGame)),
                ui::update_volume,
            ),
        )
        .run();
}
