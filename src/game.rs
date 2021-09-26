mod table; 

use crate::game::table::TableAction;
use self::table::Table;

/// Game loop and such
pub struct Game {
    table: Table
}

impl Game {
    pub fn new() -> Self {
        Self {
            table: Table::new()
        }
    }

    pub fn start(&mut self) {
        println!("Welcome to Rust Black Jack, You can't bet any money. All you can do is play. Have fun c:");
        
        loop {
            self.round();
        }
    }

    fn round(&mut self) {
        println!("--------------------------------");
        println!("Round Start. Dealing Cards, Good luck!");
        self.table.deal();

        loop {
            println!("(h) to Hit, (s) to Stand");
            let action = self.await_input_action();
            let finished = self.table.action(action);

            if finished {break;}
        }
       
    }

    
    fn await_input_action(&self) -> TableAction {
        let mut line = String::new();
       
        std::io::stdin().read_line(&mut line);

        match line.as_ref() {
            "h\n" => TableAction::Hit,
            "s\n" => TableAction::Stand,
            _ => {
                println!("{} is an invalid input", line);
                self.await_input_action()
            }
        }
    }
}