mod app_config;
mod game;
mod ingame_ui;
mod tile_loader;
mod tilemap;
mod ui;
mod undo_redo;
mod wfc;
mod particle_fx;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_hanabi::prelude::*;

use crate::undo_redo::UndoRedo;
use app_config::{GameSettings, GameState};
use game::GamePause;
use ingame_ui::AvailableTiles;
use tile_loader::load_tiles;
use tilemap::{SelectedTile, TileType, setup_grid};
use wfc::WFCState;

#[cfg(not(target_arch = "wasm32"))]
use crate::particle_fx::{setup_particle_effect, ParticleEffects};

#[cfg(target_arch = "wasm32")]
use crate::particle_fx::{fade_out_system, velocity_system};

use crate::particle_fx::spawn_on_place;
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes_override: Some(true),
        ..default()
    }));

    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: false,
    });

    // Only enable Hanabi on native platforms
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(HanabiPlugin);

    app.insert_resource(GameSettings::default());
    app.insert_resource(GamePause::default());
    app.insert_resource(AvailableTiles::default());
    app.insert_resource(SelectedTile(TileType::Empty));
    app.insert_resource(UndoRedo::default());
    app.insert_resource(WFCState::default());

    app.init_state::<GameState>();

    app.add_systems(Startup, (load_tiles, setup_grid));

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, setup_particle_effect);

    app.add_systems(
        PostStartup,
        (/*wfc::generate_level,*/ app_config::play_background_music),
    );

    app.add_systems(OnEnter(GameState::InGame), game::setup_game);

    app.add_systems(
        Update,
        (
            // UI Menus
            ui::main_menu.run_if(in_state(GameState::MainMenu)),
            // ui::settings_menu.run_if(in_state(GameState::Settings)),
            ui::load_game_menu.run_if(in_state(GameState::LoadGame)),

            // In-game systems
            game::camera_movement.run_if(in_state(GameState::InGame)),
            ingame_ui::game_menu.run_if(in_state(GameState::InGame)),
            ingame_ui::tile_panel.run_if(in_state(GameState::InGame)),
            ingame_ui::in_game_settings.run_if(in_state(GameState::InGame)),
            tilemap::place_tile_preview.run_if(in_state(GameState::InGame)),
            tilemap::update_placement_highlights
                .after(tilemap::place_tile_preview)
                .run_if(in_state(GameState::InGame)),

            ui::update_volume,

            #[cfg(target_arch = "wasm32")]
            fade_out_system, // Web-only system for sprite fallback
            #[cfg(target_arch = "wasm32")]
            velocity_system,
        ),
    );

    app.run();
}