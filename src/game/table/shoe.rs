mod deck;

use deck::Deck;
use super::rng::generate_random_number;
use super::Card;

pub struct Shoe {
    max_number_of_decks: u8,
    decks: Vec<Deck>,
}

/// The `Shoe` is the device that holds multi deck black jack games.
/// This isn't per-say a `true` shoe, but will function the same.
/// 

impl Shoe {
    pub fn new(number_of_decks: u8) -> Self {
        Self {
            max_number_of_decks: number_of_decks,
            decks: Self::build_decks(number_of_decks)
        }
    }

    fn build_decks(number_of_decks:u8)-> Vec<Deck> {
        let mut decks: Vec<Deck> = Vec::new();

        for x in 0..number_of_decks {
            decks.push(Deck::new());
        }

        decks
    }

    /// How it works is it will contain `x` number of decks. And when pulling a card from the shoe, it will randomly pick a deck and pull it out.
    /// 
    /// In the case of the shoe running out of cards. For the sake of simplicity, it will auto re-fill itself.
    pub fn pull_card(&mut self) -> Card {
        let number_of_decks = self.decks.len();

        let deck_idx = usize::from(generate_random_number(0, number_of_decks as u8));

        match self.decks[deck_idx].pop() {
            Some(card)=> card,
            None => {
                if number_of_decks == 1 {
                    // * If no decks left, generate new shoe
                    self.decks = Shoe::build_decks(self.max_number_of_decks);
                } else{
                    // * Otherwise, just remove this deck from vector
                    self.decks.remove(deck_idx);
                }
               
                self.pull_card()
            }
        }
    }
}