mod game;

use game::Game;

/// Main function to start the game of black jack
fn main() {
    let mut game = Game::new();

    game.start();
}

