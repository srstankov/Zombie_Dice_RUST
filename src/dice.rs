use rand::Rng;

pub trait Dice {
    fn roll(&self) -> char;
    fn type_id(&self) -> u8; // returns what type is the dice: 0 for green, 1 for yellow, 2 for red
}

pub struct GreenDice {
    //3*🧠, 2*👣, 1*💥
    pub sides: [char; 6], 
}

impl GreenDice {
    pub fn new() -> Self {
        GreenDice {
            sides: ['🧠', '👣', '🧠', '🧠', '💥', '👣']
        }
    }
}

impl Dice for GreenDice {
    fn roll(&self) -> char {
        let random_number = rand::thread_rng().gen_range(0..6);
        return self.sides[random_number];
    }
    fn type_id(&self) -> u8 {
        return 0;
    }
}

pub struct YellowDice {
    //2*🧠, 2*👣, 2*💥
    pub sides: [char; 6], 
}

impl YellowDice {
    pub fn new() -> Self {
        YellowDice {
            sides: ['💥', '👣', '💥', '🧠', '👣', '🧠']
        }
    }
}

impl Dice for YellowDice {
    fn roll(&self) -> char {
        let random_number = rand::thread_rng().gen_range(0..6);
        return self.sides[random_number];
    }
    fn type_id(&self) -> u8 {
        return 1;
    }
}

pub struct RedDice {
    //1*🧠, 2*👣, 3*💥
    pub sides: [char; 6], 
}

impl RedDice {
    pub fn new() -> Self {
        RedDice {
            sides: ['💥', '👣', '🧠', '💥', '💥', '👣']
        }
    }
}

impl Dice for RedDice {
    fn roll(&self) -> char {
        let random_number = rand::thread_rng().gen_range(0..6);
        return self.sides[random_number];
    }
    fn type_id(&self) -> u8 {
        return 2;
    }
}