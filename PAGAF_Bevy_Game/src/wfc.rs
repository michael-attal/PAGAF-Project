use rand::prelude::*;
use rand::distr::{self, weighted};
use rand::thread_rng;

use std::collections::VecDeque;
use bevy::prelude::*;

use crate::tile_loader::TileAssets;
use crate::tilemap::{TileMap, TileType};


const TILE_COUNT: usize = TileType::Park as usize + 1;
const WEIGHTS: [f32; TILE_COUNT] = [0.0, 3.0, 2.0, 1.0, 2.5, 1.5];

const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

const RULES: [[[bool; TILE_COUNT]; TILE_COUNT]; 4] = build_rules();

const fn build_rules() -> [[[bool; TILE_COUNT]; TILE_COUNT]; 4] {
    let mut m = [[[true; TILE_COUNT]; TILE_COUNT]; 4];
    let res = TileType::Residential as usize;
    let ind = TileType::Industrial as usize;
    let park = TileType::Park as usize;
    let mut d = 0;
    while d < 4 {
        m[d][res][ind] = false;
        m[d][ind][res] = false;
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
    /// Creates a new cell with all options enabled (except Empty)
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

    /// Sets the cell to a specific tile type
    fn set_to(&mut self, id: usize) {
        self.possible = [false; TILE_COUNT];
        self.possible[id] = true;
        self.count = 1;
        self.collapsed = true;
    }

    /// Returns the entropy (number of possibilities) of the cell
    fn entropy(&self) -> usize {
        self.count
    }
}

/// Represents the complete grid for the WFC algorithm
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

    fn idx(&self, x: usize, y: usize) -> usize {
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

    fn propagate(&mut self, sx: usize, sy: usize) -> bool {
        let mut q = VecDeque::new();
        q.push_back((sx, sy));

        while let Some((x, y)) = q.pop_front() {
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
                        q.push_back((nx, ny));
                    }
                }
            }
        }
        true
    }

    pub fn run(&mut self) -> Result<(), WFCError> {
        while let Some((x, y)) = self.lowest_entropy() {
            self.collapse(x, y);
            if !self.propagate(x, y) {
                return Err(WFCError::Contradiction);
            }
        }
        Ok(())
    }

    pub fn to_tiles(&self) -> Vec<Vec<TileType>> {
        let mut g = vec![vec![TileType::Empty; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                for i in 1..TILE_COUNT {
                    if self.cells[self.idx(x, y)].possible[i] {
                        g[y][x] = TileType::from_index(i).unwrap_or(TileType::Empty);
                        break;
                    }
                }
            }
        }
        g
    }
}

/// Bevy system for level generation
pub fn generate_level(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    mut tile_map: ResMut<TileMap>,
) {
    let mut wfc = WFCGrid::new(tile_map.width, tile_map.height);

    if wfc.run().is_err() {
        error!("WFC contradiction – map left empty");
        return;
    }

    let grid = wfc.to_tiles();
    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            let t = grid[y][x];
            tile_map.tiles[y][x].tile_type = t;
            if t != TileType::Empty {
                let handle = tile_assets.tiles[t.index()].clone();
                let entity = commands
                    .spawn((
                        SceneRoot(handle),
                        Transform::from_xyz(x as f32, 0.0, y as f32),
                    ))
                    .id();
                tile_map.entities[y][x] = Some(entity);
            }
        }
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
}