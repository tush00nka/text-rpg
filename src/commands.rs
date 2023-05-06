use std::io;

pub struct Commands;

impl Commands {

    pub fn new() -> Self {
        Self
    }

    pub fn wait_for_commands(&self) -> Vec<String> {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("[E] Failed to read command");

        command.split(' ').map(|s| s.to_string()).collect()
    }

    pub fn get_help(&self) -> String {
        "\nHELP:
        help - Show this help
        quit - Quits the game
        
        bag - Show a list of items you have

        --- cheats ---
        give [item_id] - Add an item to your bag
        ".to_string()   
    }    
}

