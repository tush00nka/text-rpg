use crate::commands::Commands;
use crate::inventory::{Inventory, ItemDatabase};
use colored::Colorize;

mod commands;
mod inventory;

fn main() {
    println!("{}", "Welcome to the (GAME)! Type 'help' for list of available actions".yellow());

    let commands = Commands::new();

    let item_database = ItemDatabase::new();
    let mut inventory = Inventory::new();

    loop {
        let command = commands.wait_for_commands(); 
        match command[0].trim() {
            "quit" => break,
            "help" => println!("{}", commands.get_help()),
            "give" => inventory.add_item(&item_database, command[1].trim().parse().unwrap()),
            "bag" => println!("{}", inventory.list()),
            _ => println!("{}", "Invalid command!!".red().bold())
        }
    }
}
