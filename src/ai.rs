use crate::game::*;

impl Game {

    fn number_of_footprints(&self, dice1_index :i8, dice2_index :i8, dice3_index :i8) -> i8 {
        let mut counter: i8 = 0;
        if dice1_index != -1 {
            counter += 1;
        }
        if dice2_index != -1 {
            counter += 1;
        }
        if dice3_index != -1 {
            counter += 1;
        }
        return counter;
    }

    fn percentage_of_dice_type_in_footprints(&self, dice_type_index: u8, dice1_index :i8, dice2_index :i8, dice3_index :i8) -> f32 { // dice_type_index indicates the type of dice and should be one of: 0 (for green), 1 (for yelow) and 2 (for red).
        let number_of_footprints = self.number_of_footprints(dice1_index, dice2_index, dice3_index);
        let mut frequency = 0;

        if dice1_index != -1 {
            let dice1 = &self.dice_cup[dice1_index as usize];
            if dice1.type_id() == dice_type_index {
                frequency += 1;
            }
        }

        if dice2_index != -1 {
            let dice2 = &self.dice_cup[dice2_index as usize];
            if dice2.type_id() == dice_type_index {
                frequency += 1;
            }
        }

        if dice3_index != -1 {
            let dice3 = &self.dice_cup[dice3_index as usize];
            if dice3.type_id() == dice_type_index {
                frequency += 1;
            }
        }

        return (100 as f32) * (frequency as f32) / (number_of_footprints as f32);
    }

    fn percentage_of_dice_type_in_cup(&self, dice_type_index: u8) -> f32 { // dice_type_index indicates the type of dice and should be one of: 0 (for green), 1 (for yelow) and 2 (for red).
        let mut frequency = 0;
        let percentage: f32;
        for i in 0 .. self.dices_indexes_in_play.len() {
            if dice_type_index == self.dice_cup[self.dices_indexes_in_play[i as usize] as usize].type_id() {
                frequency += 1;
            }
        }
        percentage = (100 as f32) * (frequency as f32) / (self.dices_indexes_in_play.len() as f32);
        return percentage;
    }

    pub fn play_ai(&self, player_index: usize, brains_collected: u8, shotguns_received: u8, dice1_index: i8, dice2_index: i8, dice3_index: i8) -> String {

        let footprints_number = self.number_of_footprints(dice1_index, dice2_index, dice3_index);
        let green_dice_percentage_in_footprints = self.percentage_of_dice_type_in_footprints(0, dice1_index, dice2_index, dice3_index);
        let yellow_dice_percentage_in_footprints = self.percentage_of_dice_type_in_footprints(1, dice1_index, dice2_index, dice3_index);
        let red_dice_percentage_in_footprints = self.percentage_of_dice_type_in_footprints(2, dice1_index, dice2_index, dice3_index);

        let dices_left_number = self.dices_indexes_in_play.len();

        // at index 1 in the diceCup the dice is green at 2 - yellow, at 3 - red, that's we use it as signifying the dice type,
	    // if the order of the dices in the diceCup are changed in a further implementation, these indexes should be checked and
	    // changed if needed

        let green_dices_left_in_cup_percentage = self.percentage_of_dice_type_in_cup(0);
        let yellow_dices_left_in_cup_percentage = self.percentage_of_dice_type_in_cup(1);
        let red_dices_left_in_cup_percentage = self.percentage_of_dice_type_in_cup(2);

        let max_brains = self.max_player_brains();
        let number_of_brains_of_player_if_stop = self.players[player_index].brains + brains_collected;

        if shotguns_received == 0 {
            if number_of_brains_of_player_if_stop >= 13 && number_of_brains_of_player_if_stop > max_brains {
                if player_index == (self.number_of_players - 1) as usize {
                    return "stop".to_string();
                }
                if green_dice_percentage_in_footprints < 50 as f32 && green_dices_left_in_cup_percentage < 25 as f32 {
                    return "stop".to_string();
                }
            }
            if max_brains >= 13 && number_of_brains_of_player_if_stop <= max_brains {
                return "roll".to_string();
            }
            if brains_collected >= 6 {
                if red_dices_left_in_cup_percentage == 100 as f32 || red_dice_percentage_in_footprints > 60 as f32 {
                    return "stop".to_string();
                }
                if green_dice_percentage_in_footprints < 50 as f32 && green_dices_left_in_cup_percentage < 20 as f32 {
                    return "stop".to_string();
                }
            }
            return "roll".to_string();
        }
    
        if shotguns_received == 1 {
            if number_of_brains_of_player_if_stop >= 13 && number_of_brains_of_player_if_stop > max_brains {
                return "stop".to_string();
            }
            if brains_collected == 0 {
                return "roll".to_string();
            }
            if max_brains >= 13 && number_of_brains_of_player_if_stop <= max_brains {
                return "roll".to_string();
            }
            if brains_collected <= 3 && brains_collected > 1 {
                if footprints_number == 0 {
                    if green_dices_left_in_cup_percentage < 20 as f32 {
                        return "stop".to_string();
                    }
                }
                if footprints_number == 3 {
                    if green_dice_percentage_in_footprints == 0  as f32{
                        return "stop".to_string();
                    }
                }
                if red_dice_percentage_in_footprints == 100 as f32 || green_dices_left_in_cup_percentage < 20 as f32 {
                    return "stop".to_string();
                }
            }
            if brains_collected > 3 {
                if footprints_number == 0 {
                    if dices_left_number > 3 && (yellow_dices_left_in_cup_percentage > 30 as f32 || red_dices_left_in_cup_percentage > 30 as f32 ) {
                        return "stop".to_string();
                    }
                    if green_dices_left_in_cup_percentage < 30 as f32 {
                        return "stop".to_string();
                    }
                } else { // footprints_number > 0
                    if yellow_dice_percentage_in_footprints > 50 as f32 || red_dices_left_in_cup_percentage > 30 as f32 {
                        return "stop".to_string();
                    }
                    if red_dice_percentage_in_footprints > 50 as f32 || green_dices_left_in_cup_percentage < 30 as f32 {
                        return "stop".to_string();
                    }
                    if footprints_number == 3 {
                        if green_dice_percentage_in_footprints == 0 as f32 {
                            return "stop".to_string();
                        }
                        if green_dice_percentage_in_footprints < 35 as f32 && brains_collected > 5 {
                            return "stop".to_string();
                        }
                    }
    
                }
            }
            return "roll".to_string();
        }
        if shotguns_received == 2 {
            if max_brains >= 13 && max_brains != self.players[player_index].brains && number_of_brains_of_player_if_stop <= max_brains {
                return "roll".to_string();
            }
            if brains_collected == 0 {
                return "roll".to_string();
            } else { // brains_collected > 0
                if number_of_brains_of_player_if_stop >= 13 && number_of_brains_of_player_if_stop > max_brains {
                    return "stop".to_string();
                }
                if brains_collected == 1 {
                    if footprints_number > 0 {
                        if footprints_number == 3 && green_dice_percentage_in_footprints == 100 as f32 {
                            return "roll".to_string();
                        }
                        if footprints_number == 2 && green_dice_percentage_in_footprints == 100 as f32 && green_dices_left_in_cup_percentage >= 33 as f32{
                            return "roll".to_string();
                        }
                        if green_dice_percentage_in_footprints == 100 as f32 && green_dices_left_in_cup_percentage >= 50 as f32 {
                            return "roll".to_string();
                        }
                    } else { // footprints_number == 0
                        if green_dices_left_in_cup_percentage >= 60 as f32 {
                            return "roll".to_string();
                        }
                    }
                    return "stop".to_string();
                }
                if brains_collected == 2 && number_of_brains_of_player_if_stop < max_brains {
                    if footprints_number == 3 && green_dice_percentage_in_footprints == 100 as f32 {
                        return "roll".to_string();
                    }
                    if footprints_number > 0 && green_dice_percentage_in_footprints == 100 as f32 && green_dices_left_in_cup_percentage == 100 as f32 {
                        return "roll".to_string();
                    }
                    if footprints_number == 0 && green_dices_left_in_cup_percentage == 100 as f32 {
                        return "roll".to_string();
                    }
                    return "stop".to_string();
                }
                return "stop".to_string();
            }
        }
        
        return "roll".to_string();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_number_of_footprints() {
        
        let new_game = super::Game::new();
        assert_eq!(new_game.number_of_footprints(1, -1, 2), 2);
        assert_eq!(new_game.number_of_footprints(-1, -1, -1), 0);
        assert_eq!(new_game.number_of_footprints(-1, -1, 5), 1);
    }

    #[test]
    fn test_percentage_of_dice_type_in_footprints() {
        
        let new_game = super::Game::new();
        assert_eq!(new_game.percentage_of_dice_type_in_footprints(0, 5, 7, -1), 50 as f32);
        assert_eq!(new_game.percentage_of_dice_type_in_footprints(1, -1, 8, -1), 100 as f32);
        assert_eq!(new_game.percentage_of_dice_type_in_footprints(2, 10, 6, 9), 33.333332);
        assert_eq!(new_game.percentage_of_dice_type_in_footprints(2, 10, 3, 9), 66.666664);
        assert_eq!(new_game.percentage_of_dice_type_in_footprints(2, -1, 2, 4), 0 as f32);
    }

    #[test]
    fn test_percentage_of_dice_type_in_cup() {
        
        let mut new_game = super::Game::new();
        assert_eq!(new_game.percentage_of_dice_type_in_cup(0), (100 as f32) * (6 as f32) / (13 as f32));
        assert_eq!(new_game.percentage_of_dice_type_in_cup(1), (100 as f32) * (4 as f32) / (13 as f32));
        assert_eq!(new_game.percentage_of_dice_type_in_cup(2), (100 as f32) * (3 as f32) / (13 as f32));
        
        new_game.dices_indexes_in_play = vec![0,2,3,4,5,6,7,8,11,12];
        assert_eq!(new_game.percentage_of_dice_type_in_cup(0), (100 as f32) * (4 as f32) / (10 as f32));
        assert_eq!(new_game.percentage_of_dice_type_in_cup(1), (100 as f32) * (4 as f32) / (10 as f32));
        assert_eq!(new_game.percentage_of_dice_type_in_cup(2), (100 as f32) * (2 as f32) / (10 as f32));
    }

    #[test]
    fn test_play_ai() {

        use crate::player::*;

        let mut new_game = super::Game::new();
        new_game.number_of_players = 3;
        new_game.players.push(Player::new("Mark", true));
        new_game.players.push(Player::new("Fernando", true));
        new_game.players.push(Player::new("Johny", true));

        assert_eq!(new_game.players[1].brains, 0); // tests player pushing

        new_game.players[0].brains = 8;
        new_game.players[1].brains = 10;
        new_game.players[2].brains = 7;

        new_game.dices_indexes_in_play = vec![0,2,3,4,5,6,7,8,11,12];
        
        // play_ai(player_index, brains_collected, shotguns_received, dice1_index, dice2_index, dice3_index)
        
        assert_eq!(new_game.play_ai(0, 0, 2, -1, -1, 9), "roll"); // 0 brains and 2 shotguns -> roll (no need to stop)
       
        assert_eq!(new_game.play_ai(0, 5, 1, 1, 10, -1), "stop"); // winning and 1 shotgun -> stop
        
        new_game.dices_indexes_in_play = vec![0,2,4,6,7,8,11,12];
        assert_eq!(new_game.play_ai(0, 4, 1, 3, 9, 5), "stop"); // 3 red dices with 👣 and 1 shotgun and 4 brains -> stop

        new_game.dices_indexes_in_play = vec![0,2,6,7,8,9,11,12];
        assert_eq!(new_game.play_ai(0, 3, 1, -1, -1, 5), "stop"); // 3 brains and 1 shotgun and small chance of green dice -> stop

        assert_eq!(new_game.play_ai(0, 3, 2, 4, 1, -1), "stop"); // 3 brains and 2 shotguns -> stop

        new_game.dices_indexes_in_play = vec![0,3,6,7,10,12];
        assert_eq!(new_game.play_ai(0, 1, 2, 4, -1, -1), "roll"); // 1 brain, 2 shotguns and big chance of green -> roll

        assert_eq!(new_game.play_ai(2, 5, 1, 4, -1, -1), "roll"); // 5 brains for Johny and 1 shotguns, high chance of green -> roll

        new_game.dices_indexes_in_play = vec![0,2,3,5,8,11]; 
        assert_eq!(new_game.play_ai(2, 1, 2, -1, -1, -1), "stop"); // small chance of green and 2 brains and 1 shotgun -> stop

        new_game.players[1].brains = 15;
        assert_eq!(new_game.play_ai(2, 8, 2, 3, 9, 5), "roll"); // equalizing current winner-candidate with 2 shotguns
                                                                // but not first to achieve 15 -> roll
        
    }
}