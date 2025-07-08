use crate::app_config::{BackgroundMusic, GameSettings, GameState, GraphicsQuality};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::wfc::WFCState;

pub struct TextBuffer(String);

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer("10".to_string())
    }
}

pub fn main_menu(
    /* mut commands: Commands,
    asset_server: Res<AssetServer>, */
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut width_text : Local<TextBuffer>,
    mut height_text : Local<TextBuffer>,
    mut wfc_state : ResMut<WFCState>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("PAGAF: Futuristic map builder");
            ui.add_space(20.0);

            if ui.button("Start Game").clicked() {
                let width: usize = width_text.0.parse().unwrap();
                let height: usize = height_text.0.parse().unwrap();
                wfc_state.reset_grid(width, height);

                next_state.set(GameState::LoadGame);
            }

            if ui.button("Settings").clicked() {
                next_state.set(GameState::Settings);
            }

            ui.text_edit_singleline(&mut width_text.0);
            ui.text_edit_singleline(&mut height_text.0);

            ui.add_space(20.0);

            if ui.button("Quit").clicked() {
                exit.write(bevy::app::AppExit::Success);
            }
        });
    });
}
pub fn settings_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
) {
    // TODO: Handle settings
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Settings");

            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                ui.set_max_width(300.0);
                ui.horizontal(|ui| {
                    ui.label("Volume:");
                    ui.add(egui::Slider::new(&mut settings.volume, 0.0..=1.0));
                });
            });

            ui.vertical_centered(|ui| {
                ui.set_max_width(300.0);
                ui.horizontal(|ui| {
                    ui.label("Brightness:");
                    ui.add(egui::Slider::new(&mut settings.brightness, 0.0..=1.0));
                });
            });

            ui.vertical_centered(|ui| {
                ui.set_max_width(300.0);
                ui.horizontal(|ui| {
                    ui.label("Graphics Quality:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", settings.graphics_quality))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut settings.graphics_quality,
                                GraphicsQuality::Low,
                                "Low",
                            );
                            ui.selectable_value(
                                &mut settings.graphics_quality,
                                GraphicsQuality::Medium,
                                "Medium",
                            );
                            ui.selectable_value(
                                &mut settings.graphics_quality,
                                GraphicsQuality::High,
                                "High",
                            );
                        });
                });
            });

            ui.add_space(20.0);

            if ui.button("Back").clicked() {
                next_state.set(GameState::MainMenu);
            }
        });
    });
}

pub fn load_game_menu(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Welcome");

            ui.add_space(20.0);

            // TODO: Handle load game & start game

            if ui.button("Load Game").clicked() {
                next_state.set(GameState::InGame);
            }

            if ui.button("New Game").clicked() {
                next_state.set(GameState::InGame);
            }

            ui.add_space(20.0);

            if ui.button("Back").clicked() {
                next_state.set(GameState::MainMenu);
            }
        });
    });
}

pub fn update_volume(
    settings: Res<GameSettings>,
    mut query: Query<&mut AudioSink, With<BackgroundMusic>>,
) {
    if settings.is_changed() {
        if let Ok(mut sink) = query.get_single_mut() {
            sink.set_volume(Volume::Linear(settings.volume));
        }
    }
}
