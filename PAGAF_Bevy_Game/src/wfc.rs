use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec;

// Each cell holds possible tile indices
#[derive(Clone)]
pub struct WFCCell {
    pub possible_tiles: Vec<usize>,
    pub collapsed: bool,
}

// Grid of cells for WFC
pub struct WFCGrid {
    pub cells: Vec<Vec<WFCCell>>,
    pub width: usize,
    pub height: usize,
}

impl WFCGrid {
    // Initialize WFC grid
    pub fn new(width: usize, height: usize, tile_count: usize) -> Self {
        let cells = vec![
            vec![
                WFCCell {
                    possible_tiles: (0..tile_count).collect(),
                    collapsed: false,
                };
                width
            ];
            height
        ];

        Self {
            cells,
            width,
            height,
        }
    }

    // Select cell with lowest entropy (fewest possibilities)
    fn lowest_entropy(&self) -> Option<(usize, usize)> {
        let mut min_entropy = usize::MAX;
        let mut cell_pos = None;

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !cell.collapsed && cell.possible_tiles.len() < min_entropy {
                    min_entropy = cell.possible_tiles.len();
                    cell_pos = Some((x, y));
                }
            }
        }

        cell_pos
    }

    // Collapse a cell randomly
    pub fn collapse_cell(&mut self, x: usize, y: usize) {
        if self.cells[y][x].collapsed || self.cells[y][x].possible_tiles.is_empty() {
            return;
        }

        let mut rng = thread_rng();
        let chosen_tile = self.cells[y][x].possible_tiles.choose(&mut rng).cloned();

        if let Some(tile) = chosen_tile {
            self.cells[y][x].possible_tiles = vec![tile];
            self.cells[y][x].collapsed = true;
        }
    }

    // Basic constraint propagation (to expand later)
    pub fn propagate(&mut self, _x: usize, _y: usize) {
        // TODO: Implement tile constraints propagation logic
    }

    // Execute one WFC step (collapse and propagate)
    pub fn step(&mut self) {
        if let Some((x, y)) = self.lowest_entropy() {
            self.collapse_cell(x, y);
            self.propagate(x, y);
        }
    }
}
