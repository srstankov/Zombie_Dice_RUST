use crate::player;
use crate::dice;
use std::io;
use std::io::Write;
use rand::Rng;
use std::{thread, time::Duration};

pub struct Game {
    pub number_of_players: u8,
    pub players: Vec<player::Player>,
    pub dice_cup: [Box<dyn dice::Dice>; 13],
    pub dices_indexes_in_play: Vec<u8>,
}

// dice_cup: [GreenDice(), GreenDice(), YellowDice(), RedDice(), GreenDice(), RedDice(), YellowDice(), GreenDice(), 
//            YellowDice(), RedDice(), GreenDice(), YellowDice(), GreenDice()]

impl Game {

    pub fn new() -> Self {

        Game {
            number_of_players: 0,
            players: Vec::new(),
            dice_cup: [Box::new(dice::GreenDice::new()), Box::new(dice::GreenDice::new()), Box::new(dice::YellowDice::new()), 
                        Box::new(dice::RedDice::new()), Box::new(dice::GreenDice::new()), Box::new(dice::RedDice::new()),
                        Box::new(dice::YellowDice::new()), Box::new(dice::GreenDice::new()), Box::new(dice::YellowDice::new()),
                        Box::new(dice::RedDice::new()), Box::new(dice::GreenDice::new()), Box::new(dice::YellowDice::new()),
                        Box::new(dice::GreenDice::new())],
            dices_indexes_in_play: vec![0,1,2,3,4,5,6,7,8,9,10,11,12], // stores indexes of the dices in the dice cup that can be rolled
            // if a dice has already been rolled before in the current move
            // we remove it from the index array
            // we also use this array to randomly pick a dice from the dice cup by using
            // the randomly picked index, we do not change anything in the diceCup array,
            // only in the index array
        }
    }

    fn start_new_game(&mut self) {
        self.number_of_players = 0;
        self.players = Vec::new();
        println!();
        println!("NEW GAME");
        println!("============================");
        println!("Welcome to Zombie Dice!");
        let mut input_line;
        let mut input_number: u8 = 0;
        while input_number < 2 || input_number > 8 {
            print!("Enter number of players: ");
            io::stdout().flush().unwrap();
            input_line = String::new();
            io::stdin().read_line(&mut input_line).expect("Not a valid string");
            let trimmed = input_line.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => input_number = i,
                Err(..) => println!("Error! Enter a number between 2 and 8!"),
            }
            if input_number < 2 || input_number > 8 {
                println!("Number of players should be between 2 and 8!");
            }
        }
        self.number_of_players = input_number;

        println!("----------------------------");
        println!("Now it's time to enter the player names. ");
        println!("If you want a player to be an AI, simply write ai after the name (e.g. Mark ai). ");
        println!("Enter the player names, each name on a different line: ");

        let mut input_name;
        let mut option;
        for _ in 0 .. self.number_of_players {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).expect("Not a valid string");
            let input_as_vector: Vec<_> = input_line.split_whitespace().collect();
            input_name = input_as_vector[0];

            if input_as_vector.len() == 2 {
                option = input_as_vector[1].to_string();
            } else {
                option = "".to_string();
            }

            if option == "ai" || option == "AI" || option == "Ai" || option == "aI" {
                self.players.push(player::Player::new(input_name, true));
            } else {
                self.players.push(player::Player::new(input_name, false));
            }
        }

        println!("----------------------------");
    }

    fn play_one_move(&mut self, player_index: usize, status: &mut i8) { // function that plays one move of a player, used later in function play()
        println!("To roll the dices and then continue your move write 'roll', to stop your move - 'stop', to exit game - 'exit'");
	    println!();
        let mut ai_string = String::new();
        if self.players[player_index].is_ai {
            ai_string = "(AI)".to_string();
        }
        print!("Next player: {} {} \n", self.players[player_index].name, ai_string);
        io::stdout().flush().unwrap();
        let mut player_input = String::new();
        let mut brains_collected :u8 = 0;
        let mut shotguns_received :u8 = 0;
        let mut dice1_index :i8 = -1;                                               // default value for a dice index, means that we should get a dice from the dice cup
        let mut dice2_index :i8 = -1;                                               // if it is not -1 it means that the dice the index represents was rolled as 👣 and we should roll it again
        let mut dice3_index :i8 = -1;                                               // therefore we update the dice index only if it is != -1
        self.dices_indexes_in_play = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];   // for every player the dice cup is refilled
       
        while player_input != "stop" && shotguns_received < 3 {
            player_input = String::new();
            if self.players[player_index].is_ai { // if the player is an AI
                thread::sleep(Duration::from_millis(4000));
                player_input = self.play_ai(player_index, brains_collected, shotguns_received, dice1_index, dice2_index, dice3_index);
                println!("> {}", player_input);

		    } else { // if the player is human
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut player_input).expect("Not a valid string");
                if let Some('\n') = player_input.chars().next_back() {
                    player_input.pop();
                }
                if let Some('\r') = player_input.chars().next_back() {
                    player_input.pop();
                }
		    }

            if player_input != "roll" && player_input != "stop" && player_input != "exit" { // checks for invalid command and makes it possible to write again a command
                println!("Command not recognised!");
                continue;
            }
            if player_input == "stop" {
                *status = 0;
                break;
            } else if player_input == "exit" {
                *status = -1; // status with value -1 signifies that the user has typed 'exit' command to exit from the current game
                break;
            } else { // playerInput = "roll"
                if self.dices_indexes_in_play.len() < 3 {
                    self.dices_indexes_in_play = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
                    println!("----------------------------");
                    println!("Dice cup refilled.");
                    println!("----------------------------");
                }
                if dice1_index == -1 { // if the last roll of the dice was not 👣
                    let random_index = rand::thread_rng().gen_range(0..self.dices_indexes_in_play.len());
                    dice1_index = self.dices_indexes_in_play[random_index] as i8; // we pick a random index from the available ones
                    self.dices_indexes_in_play.remove(random_index);              // remove it from the available ones
                }
                if dice2_index == -1 { // if the last roll of the dice was not 👣
                    let random_index = rand::thread_rng().gen_range(0..self.dices_indexes_in_play.len());
                    dice2_index = self.dices_indexes_in_play[random_index] as i8;
                    self.dices_indexes_in_play.remove(random_index); 
                }
                if dice3_index == -1 { // if the last roll of the dice was not 👣
                    let random_index = rand::thread_rng().gen_range(0..self.dices_indexes_in_play.len());
                    dice3_index = self.dices_indexes_in_play[random_index] as i8;
                    self.dices_indexes_in_play.remove(random_index); 
                }
                
                self.roll_and_update(&mut dice1_index, &mut dice2_index, &mut dice3_index, &mut brains_collected, &mut shotguns_received);
            }
        }
        self.print_move_result(brains_collected, shotguns_received, player_input, player_index as i8);
    }

    fn roll_and_update(&self, dice1_index: &mut i8, dice2_index: &mut i8, dice3_index: &mut i8, brains_collected: &mut u8,  shotguns_received: &mut u8) {
        let dice1 = &self.dice_cup[*dice1_index as usize]; // gets the actual dice from the diceCup array by the index we randomly picked
        let dice2 = &self.dice_cup[*dice2_index as usize];
        let dice3 = &self.dice_cup[*dice3_index as usize];

        let dice1_result = dice1.roll(); // we roll the dice and store the result in these variables
        let dice2_result = dice2.roll();        
        let dice3_result = dice3.roll();

        self.print_roll(*dice1_index, *dice2_index, *dice3_index, dice1_result, dice2_result, dice3_result);

        if dice1_result == '🧠' { // we check for every different possibility of the roll of each of the 3 dices
            *brains_collected += 1; // and we update the values for brains collected and shotguns received
            *dice1_index = -1;
        } else if dice1_result == '💥' {
            *shotguns_received += 1;
            *dice1_index = -1;
        }

        if dice2_result == '🧠' {
            *brains_collected += 1;
            *dice2_index = -1;
        } else if dice2_result == '💥' {
            *shotguns_received += 1;
            *dice2_index = -1;
        }

        if dice3_result == '🧠' {
            *brains_collected += 1;
            *dice3_index = -1;
        } else if dice3_result == '💥' {
            *shotguns_received += 1;
            *dice3_index = -1;
        }

        if *shotguns_received < 3 { // updates the values so that the player knows how many brains and shotguns he/she has collected till this moment
            // helps him/her decide what to choose - to continue his/hers move by 'roll' or stop by writing command 'stop'
            println!("----------------------------");
            println!("{} x 🧠 collected till now", *brains_collected);
            println!("{} x 💥 received till now", *shotguns_received);
            println!("----------------------------");
        } else {
            println!("----------------------------");
        }
    }

    fn print_move_result(&mut self, brains_collected: u8, shotguns_received: u8, player_input: String, i: i8) {
        if shotguns_received >= 3 {
            println!("💥💥💥 SHOT! No brains were eaten.");
            println!("----------------------------");
            self.print_table();
        } else if player_input == "stop" {
            println!("----------------------------");
            println!("{} x 🧠 were eaten.", brains_collected);
            println!("----------------------------");
            self.players[i as usize].brains += brains_collected; // NB!: here we update the collected brains of the player if he/she says 'stop'
            self.print_table();
        }
    }

    pub fn play(&mut self) { // the function that combines all other functions and plays the game for all players and all moves
        let mut user_input = String::new();
        println!("To start a new game -> 'new'");
	    println!("To quit -> 'quit'");
        while user_input != "new" && user_input != "quit" {
            user_input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut user_input).expect("Not a valid string");
            if let Some('\n') = user_input.chars().next_back() {
                user_input.pop();
            }
            if let Some('\r') = user_input.chars().next_back() {
                user_input.pop();
            }

            if user_input != "quit" && user_input != "new" {
                println!("Command not recognised!");
            }
        }
        if user_input == "quit" {
            return;
        }
        self.start_new_game();
        let mut has_winner = false;
        let mut status: i8;

        while !has_winner {
            for  i in 0 .. self.number_of_players {
                status = 0;
                self.play_one_move(i as usize, &mut status); // use the helpful function that plays one move of a player
                if status == -1 {                            // checks if user has typed 'exit' command to exit the current game
                    println!("----------------------------");
                    println!("Exiting current game...");
                    println!("----------------------------");
                    println!();
                    self.play();
                    return;
                }
            }

            let max_brains = self.max_player_brains(); // returns the maximum number brains a player has eaten till now

            if max_brains >= 13 { // the game has a winner
                has_winner = true;
                let winner_name = self.find_player_by_brains(max_brains);
                self.print_winner(winner_name);
                self.play(); // recursively repeat the process thus giving the opportunity to play again or quit
            }
        }
    }
    
    pub fn max_player_brains(&self) -> u8 {
        let mut max_brains: u8 = 0;
        for i in 0 .. self.number_of_players {
            if self.players[i as usize].brains > max_brains {
                max_brains = self.players[i as usize].brains;
            }
        }
        return max_brains;
    }

    fn find_player_by_brains(&self, brains: u8) -> String {
        for i in 0 .. self.number_of_players {
            if self.players[i as usize].brains == brains {
                return self.players[i as usize].name.clone();
            }
        }
        return "".to_string();
    }

    pub fn longest_name_length(&self) -> usize { // returns the maximum length of a name, used for padding in the print_table() function
        let mut max_length: usize = 0;
        let mut current_length: usize;
        for i in 0 .. self.number_of_players {
            current_length = self.players[i as usize].name.len();
            if current_length > max_length as usize {
                max_length = current_length;
            }
        }
        return max_length;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_max_player_brains() {

        use crate::player::*;

        let mut new_game = super::Game::new();
        new_game.number_of_players = 3;

        new_game.players.push(Player::new("Mark", true));
        new_game.players.push(Player::new("Fernando", true));
        new_game.players.push(Player::new("Johny", true));

        new_game.players[0].brains = 8;
        new_game.players[1].brains = 10;
        new_game.players[2].brains = 7;

        assert_eq!(new_game.max_player_brains(), 10);
    }

    #[test]
    fn test_find_player_by_brains() {

        use crate::player::*;

        let mut new_game = super::Game::new();
        new_game.number_of_players = 3;
        
        new_game.players.push(Player::new("Mark", true));
        new_game.players.push(Player::new("Fernando", true));
        new_game.players.push(Player::new("Johny", true));

        new_game.players[0].brains = 8;
        new_game.players[1].brains = 10;
        new_game.players[2].brains = 7;

        assert_eq!(new_game.find_player_by_brains(7), "Johny");
        assert_eq!(new_game.find_player_by_brains(8), "Mark");
        assert_eq!(new_game.find_player_by_brains(10), "Fernando");
    }

    #[test]
    fn test_longest_name_length() {

        use crate::player::*;

        let mut new_game = super::Game::new();
        new_game.number_of_players = 3;
        
        new_game.players.push(Player::new("Mark", true));
        new_game.players.push(Player::new("Fernando", true));
        new_game.players.push(Player::new("Johny", true));

        new_game.players[0].brains = 8;
        new_game.players[1].brains = 10;
        new_game.players[2].brains = 7;

        assert_eq!(new_game.longest_name_length(), 8);

        new_game.number_of_players += 1;
        new_game.players.push(Player::new("Davide", true));
        assert_eq!(new_game.longest_name_length(), 8);

        new_game.number_of_players += 1;
        new_game.players.push(Player::new("Michelangelo", false));
        assert_eq!(new_game.longest_name_length(), 12);
    }

    #[test]
    fn test_dice_index_removal() {

        use crate::player::*;

        let mut new_game = super::Game::new();
        new_game.number_of_players = 3;
        
        new_game.players.push(Player::new("Mark", true));
        new_game.players.push(Player::new("Fernando", true));
        new_game.players.push(Player::new("Johny", true));

        new_game.dices_indexes_in_play = vec![0,2,4,6,7,8,11,12];
        let random_index = 3;
        let dice2_index = new_game.dices_indexes_in_play[random_index] as i8;
        new_game.dices_indexes_in_play.remove(random_index); 
        assert_eq!(dice2_index, 6);
        assert_eq!(new_game.dices_indexes_in_play, vec![0,2,4,7,8,11,12]);

        let random_index = 5;
        let dice3_index = new_game.dices_indexes_in_play[random_index] as i8;
        new_game.dices_indexes_in_play.remove(random_index); 
        assert_eq!(dice3_index, 11);
        assert_eq!(new_game.dices_indexes_in_play, vec![0,2,4,7,8,12]);
    }
}