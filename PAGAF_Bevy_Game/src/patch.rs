use bevy::prelude::*;

pub fn patch_material_alpha_mode(
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_handle, mut material) in materials.iter_mut() {
        if !matches!(material.alpha_mode, AlphaMode::Opaque) {
            // Force opaque
            material.alpha_mode = AlphaMode::Opaque;
        }
    }
}