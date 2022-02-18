#[derive(Debug)]

pub struct Player {
    pub name: String,
    pub is_ai: bool,
    pub brains: u8,
}

impl Player {
    pub fn new(name: &str, is_ai: bool) -> Self {
        Player{
            name: name.to_string(),
            is_ai,
            brains: 0,
        }
    }
}
