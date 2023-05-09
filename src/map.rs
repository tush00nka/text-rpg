use colored::Colorize;
use noise::{Perlin, NoiseFn};

use crate::entity::Position;



pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    const SIZE: usize = 64;
    const SIZE_SQUARED: usize = Self::SIZE*Self::SIZE;

    const MINIMAP_SIZE: usize = 16;
    const SCALE_FACTOR: usize = Self::SIZE/Self::MINIMAP_SIZE;

    pub fn new() -> Self {

        let perlin = Perlin::new(0);

        let mut tiles = vec![TileType::Water; Self::SIZE_SQUARED];

        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                let index = x + y*Self::SIZE;
                let height = (perlin.get([(x as f64) * 0.05, (y as f64) * 0.05]) * 4.0 + 1.0).floor();
                
                if height > 3.0 {
                    tiles[index] = TileType::Mountain;
                }
                else if height > 2.0 {
                    tiles[index] = TileType::Forest;
                }
                else if height > 0.0 {
                    tiles[index] = TileType::Plain;
                }
            }
        }

        Self {
            tiles
        }
    }

    pub fn get_map(&self, player_pos: &Position) -> String {
        
        let mut map = String::new();

        for y in 0..Self::MINIMAP_SIZE {
            for x in 0..Self::MINIMAP_SIZE {

                if Position(player_pos.0/Self::SCALE_FACTOR as i32, player_pos.1/Self::SCALE_FACTOR as i32) == Position(x as i32, y as i32) {
                    map += "@";
                }
                else {
                    let index = x*Self::SCALE_FACTOR + y*Self::SCALE_FACTOR*Self::SIZE;
                    let tile = self.get_tile(index);
                    map += &format!("{tile}");
                }
            }
            map+="\n";
        }

        map
    }

    pub fn get_area(&self, position: &Position) -> String {
        let mut area = String::new();

        for y in (position.1-4)..(position.1+4) {
            if y >= 0 && y < Self::SIZE as i32 {
                for x in (position.0-4)..(position.0+4) {
                    if x >= 0 && x < Self::SIZE as i32 {
                        let index = x as usize + y as usize * Self::SIZE;
                        if &Position(x,y) == position {
                            area += "@";
                        }
                        else {
                            area += &self.get_tile(index);
                        }
                    }
                }
                area += "\n";
            }
        }

        area
    }

    pub fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn get_tile(&self, index: usize) -> String {
        match self.tiles[index] {
            TileType::Plain => {
                let char = ".".on_black().green();
                format!("{char}")
            },
            TileType::Water => {
                let char = "~".on_black().blue();
                format!("{char}")
            },
            TileType::Forest => {
                let char = "♣".on_black().green();
                format!("{char}")
            }
            TileType::Mountain => {
                let char = "^".on_black().bright_black();
                format!("{char}")
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum TileType {
    Water,
    Plain,
    Mountain,
    Forest,
}