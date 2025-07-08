use bevy::prelude::*;
use crate::tilemap::TileModelTag;

/// Recursively patch alpha mode for only the materials of entities with the TileModelTag.
pub fn patch_tile_materials_recursive_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Children, With<TileModelTag>>,
    material_query: Query<&MeshMaterial3d<StandardMaterial>>,
    children_query: Query<&Children>,
) {
    fn visit(
        entity: Entity,
        material_query: &Query<&MeshMaterial3d<StandardMaterial>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        children_query: &Query<&Children>,
    ) {
        // Patch material if MeshMaterial3d<StandardMaterial> is present
        if let Ok(mesh_material) = material_query.get(entity) {
            if let Some(mut material) = materials.get_mut(&mesh_material.0) {
                material.alpha_mode = AlphaMode::Opaque;
            }
        }

        if let Ok(children) = children_query.get(entity) {
            for &child in children {
                visit(child, material_query, materials, children_query);
            }
        }
    }

    // For each root tagged TileModelTag, traverse its hierarchy
    for children in query.iter() {
        for &child in children {
            visit(child, &material_query, &mut materials, &children_query);
        }
    }
}