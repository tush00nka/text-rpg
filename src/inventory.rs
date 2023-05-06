use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

pub struct Inventory {
    pub items: Vec<ItemSlot>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, database: &ItemDatabase, id: usize) {
        for item_slot in self.items.iter_mut() {
            if item_slot.item == database.items[id] && item_slot.item.stackable {
                item_slot.amount += 1;
                return;
            }
        }

        self.items.push(ItemSlot { item: database.items[id].clone(), amount: 1 })
    }

    pub fn list(&self) -> String {
        let mut list: String = "".to_string();
        if self.items.len() <= 0 {
            list += "\nYou've got no items in your bag!\n";
        }
        else {
            list += "\nYOUR BAG:\n";
            for i in 0..self.items.len() {
                let item_name = &self.items[i].item.item_name;
                let item_amount = &self.items[i].amount;
                let counter = i+1;

                let item_entry: String;

                if self.items[i].amount > 1{
                    item_entry = format!("{counter}. {item_name} x{item_amount}\n");                
                }
                else {
                    item_entry = format!("{counter}. {item_name}\n");  
                }

                list += &item_entry;
            }
        }

        list
    }
}

#[derive(Deserialize)]
pub struct ItemDatabase {
    pub items: Vec<Item>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        
        let mut path: String = "".to_string();
        
        let _ = File::open("item_database.json")
            .expect("[E] Could not open the database file").read_to_string(&mut path);

        serde_json::from_str(&path).unwrap()
    }
}

pub struct ItemSlot {
    pub item: Item,
    pub amount: u32,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct Item {
    pub item_id: usize,
    pub item_name: String,
    pub item_description: String,
    pub item_type: ItemType,
    pub stackable: bool,
}

#[derive(Deserialize, PartialEq, Clone)]
pub enum ItemType {
    MeleeWeapon = 0,
    RangedWeapon = 1,
    Food = 2,
    Resource = 3,
}
