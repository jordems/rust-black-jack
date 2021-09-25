mod suite; 
mod kind; 

use self::{kind::{Kind, index_to_kind}, suite::{Suite, index_to_suite}};


/// The implementation of a Single Card of a deck
#[derive(Debug, Clone, Copy)]
pub struct Card {
    /// The suite of the card (eg. Hearts)
    suite: Suite,
    /// The kind of the card (eg. Ace)
    kind: Kind
}

impl Card {
    /// Creates a new card with the specified `Suite` and `Kind`
    pub fn new(suite_index: u8, kind_index: u8) -> Self {
        Self {
            suite: index_to_suite(suite_index),
            kind: index_to_kind(kind_index)
        }
    }

    /// @returns the `Suite` of the card
    pub fn get_suite(&self) -> Suite {
        self.suite
    }

    /// @returns the `Kind` of the card
    pub fn get_kind(&self) -> Kind {
        self.kind
    }
}


