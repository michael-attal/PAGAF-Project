use crate::tile_loader::TileAssets;
use crate::tilemap::{TileMap, TileType};
use bevy::prelude::*;
use rand::distr::weighted;
use rand::prelude::*;
use rand::thread_rng;
use std::collections::VecDeque;

/* ─────────────────────────────  Constants  ──────────────────────────────── */

const TILE_COUNT: usize = TileType::Park as usize + 1;
const WEIGHTS: [f32; TILE_COUNT] = [0.0, 3.0, 2.0, 1.0, 2.5, 1.5];

const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

// Règles de placement des tuiles
const RULES: [[[bool; TILE_COUNT]; TILE_COUNT]; 4] = build_rules();

const fn build_rules() -> [[[bool; TILE_COUNT]; TILE_COUNT]; 4] {
    let mut m = [[[true; TILE_COUNT]; TILE_COUNT]; 4];
    let res = TileType::Residential as usize;
    let ind = TileType::Industrial as usize;
    let park = TileType::Park as usize;
    let mut d = 0;
    while d < 4 {
        // Residential and industrial cannot be adjacent
        m[d][res][ind] = false;
        m[d][ind][res] = false;
        // Parks cannot be adjacent to each other
        m[d][park][park] = false;
        d += 1;
    }
    m
}

#[derive(Debug)]
pub enum WFCError {
    Contradiction,
    InvalidState,
}

/// Represents a cell in the Wave Function Collapse algorithm
#[derive(Clone, Debug)]
pub struct WFCCell {
    pub possible: [bool; TILE_COUNT],
    pub count: usize,
    pub collapsed: bool,
}

impl WFCCell {
    fn new_full() -> Self {
        let mut p = [false; TILE_COUNT];
        let mut c = 0;
        for i in 1..TILE_COUNT {
            p[i] = true;
            c += 1;
        }
        Self {
            possible: p,
            count: c,
            collapsed: false,
        }
    }

    fn set_to(&mut self, id: usize) {
        self.possible = [false; TILE_COUNT];
        self.possible[id] = true;
        self.count = 1;
        self.collapsed = true;
    }

    fn entropy(&self) -> usize {
        self.count
    }
}

#[derive(Resource)]
pub struct WFCState {
    pub grid: WFCGrid,
}

impl Default for WFCState {
    fn default() -> Self {
        Self {
            grid: WFCGrid::new(50, 50), // Same size as TileMap // TODO: refactor it
        }
    }
}

pub struct WFCGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<WFCCell>,
}

impl WFCGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![WFCCell::new_full(); width * height],
        }
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn lowest_entropy(&self) -> Option<(usize, usize)> {
        let mut min = usize::MAX;
        let mut candidates = Vec::new();

        self.cells.iter().enumerate().for_each(|(idx, cell)| {
            if !cell.collapsed && cell.count > 1 {
                let x = idx % self.width;
                let y = idx / self.width;

                match cell.count.cmp(&min) {
                    std::cmp::Ordering::Less => {
                        min = cell.count;
                        candidates = vec![(x, y)];
                    }
                    std::cmp::Ordering::Equal => {
                        candidates.push((x, y));
                    }
                    _ => {}
                }
            }
        });

        candidates.choose(&mut thread_rng()).copied()
    }

    fn collapse(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        let mut choice = Vec::<usize>::new();
        let mut weight = Vec::<f32>::new();

        for i in 1..TILE_COUNT {
            if self.cells[idx].possible[i] {
                choice.push(i);
                weight.push(WEIGHTS[i]);
            }
        }

        let dist = weighted::WeightedIndex::new(&weight).unwrap();
        let pick = choice[dist.sample(&mut thread_rng())];
        self.cells[idx].set_to(pick);
    }

    pub fn propagate(&mut self, sx: usize, sy: usize) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy));

        while let Some((x, y)) = queue.pop_front() {
            let idx = self.idx(x, y);
            for dir in 0..4 {
                if let Some((nx, ny)) = neighbour(self.width, self.height, x, y, dir) {
                    let nidx = self.idx(nx, ny);
                    let mut changed = false;

                    for t in 1..TILE_COUNT {
                        if !self.cells[nidx].possible[t] {
                            continue;
                        }
                        let mut ok = false;
                        for s in 1..TILE_COUNT {
                            if self.cells[idx].possible[s] && RULES[dir][s][t] {
                                ok = true;
                                break;
                            }
                        }
                        if !ok {
                            self.cells[nidx].possible[t] = false;
                            self.cells[nidx].count -= 1;
                            changed = true;
                        }
                    }

                    if self.cells[nidx].count == 0 {
                        return false;
                    }
                    if changed {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        true
    }

    pub fn place_tile(&mut self, x: usize, y: usize, tile_type: TileType) -> bool {
        let idx = self.idx(x, y);
        if self.cells[idx].collapsed {
            return false;
        }

        self.cells[idx].set_to(tile_type as usize);
        self.propagate(x, y)
    }

    pub fn can_place_tile(&self, x: usize, y: usize, tile_type: TileType) -> bool {
        let idx = self.idx(x, y);
        if self.cells[idx].collapsed {
            return false;
        }

        // Checks whether the tile respects the rules with its neighbors
        for dir in 0..4 {
            if let Some((nx, ny)) = neighbour(self.width, self.height, x, y, dir) {
                let nidx = self.idx(nx, ny);
                if self.cells[nidx].collapsed {
                    let mut valid = false;
                    for t in 1..TILE_COUNT {
                        if self.cells[nidx].possible[t] && RULES[dir][tile_type as usize][t] {
                            valid = true;
                            break;
                        }
                    }
                    if !valid {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_possible_tiles(&self, x: usize, y: usize) -> Vec<TileType> {
        let idx = self.idx(x, y);
        let mut possible = Vec::new();

        if !self.cells[idx].collapsed {
            for i in 1..TILE_COUNT {
                if self.cells[idx].possible[i] {
                    if let Some(tile_type) = TileType::from_index(i) {
                        possible.push(tile_type);
                    }
                }
            }
        }

        possible
    }
}

/// Returns the neighbor's coordinates in the specified direction
fn neighbour(w: usize, h: usize, x: usize, y: usize, dir: usize) -> Option<(usize, usize)> {
    match dir {
        NORTH if y > 0 => Some((x, y - 1)),
        SOUTH if y + 1 < h => Some((x, y + 1)),
        EAST if x + 1 < w => Some((x + 1, y)),
        WEST if x > 0 => Some((x - 1, y)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wfc_cell_new() {
        let cell = WFCCell::new_full();
        assert_eq!(cell.count, TILE_COUNT - 1);
        assert!(!cell.collapsed);
    }

    #[test]
    fn test_neighbour_boundaries() {
        assert_eq!(neighbour(3, 3, 0, 0, NORTH), None);
        assert_eq!(neighbour(3, 3, 2, 2, SOUTH), None);
        assert!(neighbour(3, 3, 1, 1, EAST).is_some());
    }

    #[test]
    fn test_can_place_tile() {
        let mut grid = WFCGrid::new(3, 3);

        // Initial placement test
        assert!(grid.can_place_tile(1, 1, TileType::Residential));

        // Place a tile and check constraints
        grid.place_tile(1, 1, TileType::Residential);
        assert!(!grid.can_place_tile(1, 1, TileType::Industrial));
    }
}
