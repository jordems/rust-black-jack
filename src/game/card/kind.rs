

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Kind {
    fn to_index(&self) -> u8 {
        kind_to_index(*self)
    }
}


pub fn index_to_kind(index:u8) -> Kind {
    match index {
        0 => Kind::Two,
        1 => Kind::Three,
        2 => Kind::Four,
        3 => Kind::Five,
        4 => Kind::Six,
        5 => Kind::Seven,
        6 => Kind::Eight,
        7 => Kind::Nine,
        8 => Kind::Ten,
        9 => Kind::Jack,
        10 => Kind::Queen,
        11 => Kind::King,
        12 => Kind::Ace,
        _ => panic!("Invalid kind index found"),
    }
}

pub fn kind_to_index(kind: Kind)-> u8 {
    match kind {
        Kind::Two => 0,
        Kind::Three => 1,
        Kind::Four => 2,
        Kind::Five => 3,
        Kind::Six => 4,
        Kind::Seven => 5,
        Kind::Eight => 6,
        Kind::Nine => 7,
        Kind::Ten => 8,
        Kind::Jack => 9,
        Kind::Queen => 10,
        Kind::King => 11,
        Kind::Ace => 12,
    }
}

