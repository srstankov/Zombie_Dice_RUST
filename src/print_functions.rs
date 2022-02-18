use crate::game::*;

impl Game {

    pub fn print_roll(&self, dice1_index: i8, dice2_index: i8, dice3_index: i8, dice1_result: char, dice2_result: char, dice3_result: char) {
        let dice1 = &self.dice_cup[dice1_index as usize];
        let dice2 = &self.dice_cup[dice2_index as usize];
        let dice3 = &self.dice_cup[dice3_index as usize];

        println!("----------------------------");
	    println!("Dice roll: ");

        if dice1.type_id() == 0 {
            println!("{} <--- {}", dice1_result, "🟩");
        } else if dice1.type_id() == 1 {
            println!("{} <--- {}", dice1_result, "🟨");
        } else {
            println!("{} <--- {}", dice1_result, "🟥");
        }

        if dice2.type_id() == 0 {
            println!("{} <--- {}", dice2_result, "🟩");
        } else if dice2.type_id() == 1 {
            println!("{} <--- {}", dice2_result, "🟨");
        } else {
            println!("{} <--- {}", dice2_result, "🟥");
        }

        if dice3.type_id() == 0 {
            println!("{} <--- {}", dice3_result, "🟩");
        } else if dice3.type_id() == 1 {
            println!("{} <--- {}", dice3_result, "🟨");
        } else {
            println!("{} <--- {}", dice3_result, "🟥");
        }
    }

    pub fn print_table(&self) { // prints the results of all players
        println!("Results so far: \n");
        let max_name_length = self.longest_name_length();
        println!("{}", format!("+{}+", "-".repeat(max_name_length + 13)));
        println!("| {:width$} | {brains} |", "Name", width = max_name_length + 2, brains = "Brains");
        for  i in 0 .. self.number_of_players {
            println!("{}", format!("|{}|", "-".repeat(max_name_length + 13)));
            if self.players[i as usize].brains < 10 {
                println!("| {name:width$} |   {brains}    |", name = self.players[i as usize].name, 
                            width = max_name_length + 2, brains = self.players[i as usize].brains);
            } else {
                println!("| {name:width$} |   {brains}   |", name = self.players[i as usize].name, 
                            width = max_name_length + 2, brains = self.players[i as usize].brains);
            }
        }
        println!("{}", format!("+{}+\n", "-".repeat(max_name_length + 13)));
    }

    pub fn info(&self) { // prints useful general info for the players at he beginning of tha game
        println!();
        println!("Welcome to Zombie Dice!");
        println!("Here are some important commands and notes before you play...");
        println!("To set the game at the begining use the following commands:");
        println!("To start a new game -> 'new'");
        println!("To quit -> 'quit'");
        println!();
        println!("Then add the number of players and their names.");
        println!("You can choose a certain player to be an AI by writing ai after the name (e.g. Mark ai)");
        println!("After setting the game, use the following commands:");
        println!("To roll the dices and then continue your move write 'roll', to stop your move - 'stop'");
        println!();
        println!("Note that if you have rolled 👣 on some dices they will be included in your next roll");
        println!("if you decide to continue your move with the command 'roll'. The dices to roll will again be 3");
        println!("as dices from the dice cup will be added if necessary.");
        println!("If there are less than 3 dices in the dice cup, it will be refilled.");
        println!("The player which has the most brains and those brains are >= 13 wins. If there are two or more players with the same ");
        println!("result, the player who first reached the number of brains wins.");
        println!()
    }

    pub fn print_winner(&self, winner_name: String) {
        println!();
        println!("============================");
        println!("WINNER:");
        println!("{}", winner_name);
        println!("============================");
        println!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dice_type_id() {

        let new_game = super::Game::new();

        let dice_green_4 = &new_game.dice_cup[4];
        let dice_yellow_6 = &new_game.dice_cup[6];
        let dice_red_5 = &new_game.dice_cup[5];

        assert_eq!(dice_green_4.type_id(), 0);
        assert_eq!(dice_yellow_6.type_id(), 1);
        assert_eq!(dice_red_5.type_id(), 2);

        let dice_green_12 = &new_game.dice_cup[4];
        let dice_yellow_8 = &new_game.dice_cup[6];
        let dice_red_3 = &new_game.dice_cup[5];

        assert_eq!(dice_green_12.type_id(), 0);
        assert_eq!(dice_yellow_8.type_id(), 1);
        assert_eq!(dice_red_3.type_id(), 2);
    }
}