use crate::commands::Commands;
use crate::entity::Position;
use crate::inventory::{Inventory, ItemDatabase};
use crate::map::Map;
use crate::player::Player;
use colored::Colorize;

mod commands;
mod inventory;
mod map;
mod player;
mod entity;

fn main() {
    println!("{}", "Welcome to [TEXT QUEST]! Type 'help' for list of available actions".yellow());

    let commands = Commands::new();

    let item_database = ItemDatabase::new();
    let mut inventory = Inventory::new();

    let map = Map::new();

    let mut player = Player::new(Position(0,0));

    loop {
        let command = commands.wait_for_commands(); 
        match command[0].trim() {
            "quit" => break,
            "help" => println!("{}", commands.get_help()),

            "give" => inventory.add_item(&item_database, command[1].trim().parse().unwrap()),
            "desc" => println!("{}", inventory.get_description(command[1].trim().parse().unwrap())),
            "bag" => println!("{}", inventory.list()),

            "map" => println!("{}", map.get_map(&player.position)),
            "area" => println!("{}", map.get_area(&player.position)),
            
            "w" => player.go(&map, 'w'),
            "a" => player.go(&map, 'a'),
            "s" => player.go(&map, 's'),
            "d" => player.go(&map, 'd'),
            _ => println!("{}", "Invalid command!!".red().bold())
        }
    }
}
