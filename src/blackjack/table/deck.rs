

use super::Card;
use super::rng::generate_random_number;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    
    /// Generates a new deck of `52` cards 
    ///
    /// **Generation strategy**
    ///
    /// This strategy means that we will be storing the cards in a random order in memory at deck creation time.
    /// Then just pop a card out of the list when we want another card. 
    /// An opposing strategy would be to generate a random number between 0-[# of cards left] every time you want a new card.
    /// The latter being what would probably be used in an actual official gambling application.
    ///
    /// - Generate one of every card in the deck in an array of size 52 called `s`
    /// - Create a random sequence of numbers from 0 - 51 in an array of size 52 called `r`
    /// - Create a new array `d` for x in 0..51: `d[r[x]] = s[x]`
    pub fn new() -> Self {
        Self {
            cards: Self::build_deck()
        }
    }

    /// Pulls a card off the `Deck`
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    // @returns the number of cards left in the `Deck`
    pub fn size(&self) -> usize {
        self.cards.len()
    }


    /// Builds the `cards` in the deck
    ///
    /// - Generate one of every card in the deck in an array of size 52 called `s`
    /// - Create a random sequence of numbers from 0 - 51 in an array of size 52 called `r`
    /// - Create a new array `d` for x in 0..51: `d[r[x]] = s[x]` 
    fn build_deck() -> Vec<Card> {
        let ordered_cards = Self::build_ordered_cards();
        let random_sequence = Self::build_random_sequence();

        let mut cards = Vec::with_capacity(52);

        for index in 0..52 {
            let random_number = usize::from(random_sequence[index]);

            cards.push(ordered_cards[random_number])
        }

        cards
    }

    /// Generates one of every card in the deck in an array of size `52`
    fn build_ordered_cards() -> Vec<Card> {
        let mut ordered_cards =  Vec::with_capacity(52);

        for kind_index in 0..13 {
            for suite_index in 0..4 {
                ordered_cards.push(Card::new(suite_index, kind_index));
            }
        }

        ordered_cards
    }

    /// Create a random sequence of numbers from 0 - 51 in an array of size `52`
    fn build_random_sequence() -> Vec<u8> {
        let mut numbers: Vec<u8> =  Vec::with_capacity(52);
        let mut sequence: Vec<u8> =  Vec::with_capacity(52);

        for index in 0..52 {
            numbers.push(index);
        }

        for index in 0..52 {
            let random_idx = usize::from(generate_random_number(0, 52-index));

            sequence.push(numbers.remove(random_idx));
        }

        sequence
    }
}

