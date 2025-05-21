use shark_chess_ai::Board;
use shark_chess_ai::Game;

fn main() {
    
    let game = Game::initialize();

    game.to_string();
    println!("{}", game.to_string());
}
