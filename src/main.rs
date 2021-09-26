mod blackjack;

use blackjack::Game;

fn main() {
    let mut game = Game::new();

    game.start();

}

