use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    LoadGame,
    Settings,
    InGame,
}

#[derive(Resource)]
pub struct GameSettings {
    pub volume: f32,
    pub graphics_quality: GraphicsQuality,
    pub brightness: f32,
}

#[derive(Debug, PartialEq)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            volume: 0.5,
            graphics_quality: GraphicsQuality::Medium,
            brightness: 0.7,
        }
    }
}
