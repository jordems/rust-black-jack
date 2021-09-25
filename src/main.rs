
#[path="game/deck.rs"] mod deck; use deck::Deck;

fn main() {
    let mut deck = Deck::new();

    for _ in 0..deck.size() {
        println!("Pulled Card {:?}",deck.pop());
    }

    
}

