
mod game;
mod utils;
mod rayattacks;

use game::*;


fn main() {
    
    let game = Game::initialize();

    game.to_string();
    println!("{}", game.to_string());
    println!("{}, {}, {} ", game.en_passant.unwrap_or_default(), game.halfmove_clock, game.fullmove_number);
}
