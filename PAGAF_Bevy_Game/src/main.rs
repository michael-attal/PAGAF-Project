mod app_config;
mod ui;
mod game;
mod ingame_ui;
mod tilemap;

use crate::app_config::{GameSettings, GameState};
use crate::ingame_ui::AvailableTiles;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy::audio::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(game::GamePause::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .init_state::<GameState>()
        .insert_resource(GameSettings::default())
        .insert_resource(AvailableTiles::default())
        .add_systems(Startup, app_config::play_background_music)
        .add_systems(Update, ui::update_volume)
        .add_systems(Update, ui::main_menu.run_if(in_state(GameState::MainMenu)))
        .add_systems(
            Update,
            ui::settings_menu.run_if(in_state(GameState::Settings)),
        )
        .add_systems(
            Update,
            ui::load_game_menu.run_if(in_state(GameState::LoadGame)),
        )
        .add_systems(OnEnter(GameState::InGame), game::setup_game)
        .add_systems(
            Update,
            (
                ingame_ui::game_menu,
                ingame_ui::tile_panel,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(Update, game::rotate_cube
            .run_if(in_state(GameState::InGame))
            .run_if(not(|pause: Res<game::GamePause>| pause.paused))
        )
        .run();
}