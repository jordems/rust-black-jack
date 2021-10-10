mod shoe; 
mod player;
mod card;
mod rng;

use shoe::Shoe;
use player::Player;
use card::{Card, Kind};

pub enum TableAction {
    Hit,
    Stand,
}

pub struct Table {
    shoe: Shoe,
    dealer: Player,
    player: Player,
}

impl Table {

    /// Creates a new table with a deck of cards 
    pub fn new() -> Self {
        Self {
            shoe: Shoe::new(4),
            dealer: Player::new(),
            player: Player::new(),
        }
    }

    /// Pulls cards for player and dealer, and sets
    pub fn deal(&mut self){
        let player_cards= [self.shoe.pull_card(), self.shoe.pull_card()];
        for card in player_cards {
            self.player.add_card_to_hand(card);
        }

        let dealer_cards = [self.shoe.pull_card(), self.shoe.pull_card()];
        for card in dealer_cards {
            self.dealer.add_card_to_hand(card)
        }

        println!("The dealer is showing: {:?}", dealer_cards[0]);
        println!("Your cards: ({}){:?}", self.player.evaluate_hand(), player_cards);
    }

    /// Perform action, and if that ends the round, the `evaluate_round`
    ///
    /// @returns finished - If the round has finished from the action its `true`
    pub fn action(&mut self, action: TableAction) -> bool {
        match action {
            TableAction::Hit => {
                let card = self.shoe.pull_card();
                self.player.add_card_to_hand(card);
                println!("Card pulled: {:?}", card);

                let score = self.player.evaluate_hand();                
                if score > 21 {
                    println!("You've busted with a '{}', oh geez", score);
                    self.clean_round();
                    return true;
                }

                println!("Your cards: ({}){:?}", self.player.evaluate_hand(), self.player.get_hand());

                false
            },
            TableAction::Stand => {
                self.evaluate_round();
                true
            }
        }
    }

    // Reveals dealers cards, then dealer hits up to 17
    fn evaluate_round(&mut self){
        let mut dealer_score = self.dealer.evaluate_hand();
        println!("Dealer's hidden card was {:?} making {}", self.dealer.get_hand()[1], dealer_score);

        loop {
            let card = self.shoe.pull_card();
            self.dealer.add_card_to_hand(card);
            println!("Dealer pulls a {:?}", card);
            dealer_score = self.dealer.evaluate_hand();

            if dealer_score >= 17 {
                break;
            }
        }

        let player_score = self.player.evaluate_hand();

        if dealer_score > 21 {
            println!("The dealer busted with a {}!!! You win!", dealer_score);
        }else if dealer_score == player_score {
            println!("The dealer pushes. Nice Tie!");
        }else if dealer_score > player_score {
            println!("The dealer wins with {} against your {}",dealer_score, player_score);
        }else {
            println!("You win with a {} against the dealers {}", player_score, dealer_score);
        }
        self.clean_round();
    }


    fn clean_round(&mut self) {
        self.player.clean();
        self.dealer.clean();
    }
}