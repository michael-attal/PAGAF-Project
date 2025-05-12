use crate::app_config::{GameSettings, GameState, GraphicsQuality};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn main_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("PAGAF: Futuristic map builder");
            ui.add_space(20.0);

            if ui.button("Start Game").clicked() {
                next_state.set(GameState::LoadGame);
            }

            if ui.button("Settings").clicked() {
                next_state.set(GameState::Settings);
            }

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
