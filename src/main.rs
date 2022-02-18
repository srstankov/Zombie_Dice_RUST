mod player;
mod dice;
mod game;
mod print_functions;
mod ai;

fn main() {

    let mut new_game: game::Game = game::Game::new();
    new_game.info();
    new_game.play();
}