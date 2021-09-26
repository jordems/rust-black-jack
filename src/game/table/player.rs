use super::{Card, Kind};

pub struct Player {
    hand: Vec<Card>
}

impl Player {

    /// Creates a new `Player` with an empty hand
    pub fn new() -> Self {
        Self {
            hand: Vec::with_capacity(15)
        }
    }

    /// Adds a card to the players active hand
    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    /// Gets the active hand of the player
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

    /// Evaluates the score of a `Players` hand
    ///
    /// Handling the case of `Ace`'s
    pub fn evaluate_hand(&self) -> u8 {
        let mut soft_score: u8 = 0;
        let mut contains_ace: bool = false;

        for card in self.hand.iter() {
            soft_score += card.get_score();

            if matches!(card.get_kind(), Kind::Ace) {
                contains_ace = true;
            }
        }

        if contains_ace {
            let hard_score: u8 = soft_score + 10;

            if hard_score <= 21 {
                return hard_score;
            }
        }

        soft_score   
    }

    // Cleans the player's hand
    pub fn clean(&mut self) {
        self.hand.clear();
    }
}