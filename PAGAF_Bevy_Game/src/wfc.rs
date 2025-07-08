use crate::tilemap::{TileMap, TileType};
use bevy::prelude::*;
use bevy_ghx_proc_gen::bevy_ghx_grid::ghx_grid::cartesian::coordinates::Cartesian2D;
use bevy_ghx_proc_gen::bevy_ghx_grid::ghx_grid::cartesian::grid::CartesianGrid;
use bevy_ghx_proc_gen::proc_gen::generator::builder::GeneratorBuilder;
use bevy_ghx_proc_gen::proc_gen::generator::Generator;
use bevy_ghx_proc_gen::proc_gen::generator::model::{ModelCollection, ModelIndex, ModelRotation};
use bevy_ghx_proc_gen::proc_gen::generator::observer::{GenerationUpdate, QueuedObserver};
use bevy_ghx_proc_gen::proc_gen::generator::rules::RulesBuilder;
use bevy_ghx_proc_gen::proc_gen::generator::socket::{SocketCollection, SocketsCartesian2D};
use bevy_ghx_proc_gen::proc_gen::ghx_grid::cartesian::coordinates::CartesianPosition;
use bevy_ghx_proc_gen::proc_gen::ghx_grid::grid::GridIndex;
use rand::prelude::*;

/* ─────────────────────────────  Constants  ──────────────────────────────── */

const SELECTION_ID_TO_TYPE_MAP: [TileType; 8] = [
    TileType::Road,
    TileType::Road,
    TileType::Road,
    TileType::Road,
    TileType::Residential,
    TileType::Commercial,
    TileType::Industrial,
    TileType::Park
];

const ID_TO_TYPE_MAP: [TileType; 8] = [
    TileType::Road,
    TileType::RoadL,
    TileType::RoadT,
    TileType::RoadX,
    TileType::Residential,
    TileType::Commercial,
    TileType::Industrial,
    TileType::Park
];

#[derive(Resource)]
pub struct WFCState {
    pub grid: WFCGrid,
}

impl Default for WFCState {
    fn default() -> Self {
        Self {
            grid: WFCGrid::new(20, 20), // Same size as TileMap // TODO: refactor it
        }
    }
}

pub struct WFCGrid {
    pub width: usize,
    pub height: usize,
    pub observer : QueuedObserver,

    generator : Generator<Cartesian2D, CartesianGrid<Cartesian2D>>
}

impl WFCGrid {
    pub fn new(width: usize, height: usize) -> Self {
        // A SocketCollection is what we use to create sockets and define their connections
        let mut sockets = SocketCollection::new();
        let (road, road_intersection, road_side) = (
            sockets.create(), sockets.create(), sockets.create()
        );

        let (residential, commercial, industrial, park, urban) = (
            sockets.create(), sockets.create(), sockets.create(), sockets.create(), sockets.create()
        );

        sockets.add_connection(road, [road, road_intersection]);
        sockets.add_connection(road_side, [residential, industrial, park]);
        sockets.add_connection(residential, [residential]);
        sockets.add_connection(commercial, [commercial]);
        sockets.add_connection(industrial, [industrial]);
        sockets.add_connection(park, [park, residential]);
        sockets.add_connection(urban, [road_side, residential, park]);

        let mut models = ModelCollection::<Cartesian2D>::new();

        // Road Models: Simple, Turns, T intersections and Crossways
        models.create(SocketsCartesian2D::Simple {
            x_pos: road,
            x_neg: road,
            y_pos: road_side,
            y_neg: road_side,
        }).with_additional_rotation(ModelRotation::Rot90);
        models.create(SocketsCartesian2D::Simple {
            x_pos: road_intersection,
            x_neg: road_side,
            y_pos: road_side,
            y_neg: road_intersection,
        }).with_all_rotations().with_weight(0.1);
        models.create(SocketsCartesian2D::Simple {
            x_pos: road_intersection,
            x_neg: road_intersection,
            y_pos: road_intersection,
            y_neg: road_side,
        }).with_all_rotations().with_weight(0.1);
        models.create(SocketsCartesian2D::Mono(road_intersection)).with_weight(0.1);

        // Building Models
        // Residentials are more or less universal
        models.create(SocketsCartesian2D::Mono(residential)).with_weight(3.0);

        // Commercials must be at the end of a road and make gigantic complexes
        models.create(SocketsCartesian2D::Multiple {
            x_pos: Vec::from([commercial, urban]),
            x_neg: Vec::from([commercial, urban]),
            y_pos: Vec::from([commercial, road]),
            y_neg: Vec::from([commercial, urban]),
        }).with_additional_rotation(ModelRotation::Rot90).with_weight(0.2);

        // Industrials should be kept in their own sections
        models.create(SocketsCartesian2D::Mono(industrial)).with_weight(1.0);

        // Parks need at least a single residential tile
        models.create(SocketsCartesian2D::Mono(park)).with_weight(1.0);

        // We give the models and socket collection to a RulesBuilder and get our Rules
        let rules = RulesBuilder::new_cartesian_2d(models, sockets).build().unwrap();

        // This is the base grid on which the city is built
        let grid = CartesianGrid::new_cartesian_2d(width as u32, height as u32, false, false);

        let mut generator = GeneratorBuilder::new()
            .with_rules(rules)
            .with_grid(grid)
            .build()
            .unwrap();

        // The observer is how the rest of the application will learn of WFC's actions
        // It registers changes on which we can act later on. For a concrete example, see tilemap.rs
        let mut observer = QueuedObserver::new(&mut generator);

        Self {
            width,
            height,
            generator,
            observer
        }
    }

    pub fn place_tile(&mut self, x: usize, y: usize, tile_type: TileType) -> bool {
        let x = x as u32;
        let y = y as u32;

        let index = self.generator.grid().get_index_2d(x, y);
        let available_models = self.generator.get_models_on(index);

        for i in 0..available_models.len()
        {
            if SELECTION_ID_TO_TYPE_MAP[available_models[i].model_index] == tile_type {
                self.generator.set_and_propagate(index, available_models[i], false);
                return true
            }
        }

        false
    }

    pub fn can_place_tile(&self, x: usize, y: usize, tile_type: TileType) -> bool {
        let x = x as u32;
        let y = y as u32;

        let index = self.generator.grid().get_index_2d(x, y);
        let available_models = self.generator.get_models_on(index);

        for i in 0..available_models.len()
        {
            if SELECTION_ID_TO_TYPE_MAP[available_models[i].model_index] == tile_type {
                return true;
            }
        }

        false
    }

    pub fn collapsed(&self, x: usize, y: usize) -> bool {
        let x = x as u32;
        let y = y as u32;

        let index = self.generator.grid().get_index_2d(x, y);
        let available_models = self.generator.get_models_on(index);

        available_models.len() <= 1
    }

    pub fn pos_from_index(&self, grid_index: GridIndex) -> CartesianPosition {
        self.generator.grid().pos_from_index(grid_index)
    }
    
    pub fn tiletype_from_id(&self, id: ModelIndex) -> TileType {
        ID_TO_TYPE_MAP[id]
    }
    
    pub fn collapse_all(&mut self) -> bool {
        self.generator.generate().is_ok()
    }
}
