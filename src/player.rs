use colored::Colorize;

use crate::{entity::Position, map::Map};

pub struct Player {
    pub position: Position,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            position
        }
    }

    pub fn go(&mut self, map: &Map, to: char) {

        let delta = match to {
            'w' => Position(0, -1),
            'a' => Position(-1, 0),
            's' => Position(0, 1),
            'd' => Position(1, 0),
            _ => Position(0, 0)
        };

        let end_pos = self.position + delta;
        if end_pos.0 >= 0 && end_pos.0 < map.get_size() as i32 &&
           end_pos.1 >= 0 && end_pos.1 < map.get_size() as i32 {
            self.position = end_pos;
            println!("{}", map.get_area(&end_pos));
        }
        else {
            println!("{}", "Can't go here!".yellow());
        }
    }
}