#[derive(Debug, Clone, Copy)]
pub enum Suite {
    Hearts,
    Clubs,
    Spades,
    Diamonds
}

impl Suite {
    fn to_index(&self) -> u8 {
        suite_to_index(*self)
    }
}


pub fn index_to_suite(index: u8) -> Suite {
    match index{
        0 => Suite::Clubs,
        1 => Suite::Diamonds,
        2 => Suite::Hearts,
        3 => Suite::Spades,
        _ => panic!("Invalid suite index found")
    }
}

pub fn suite_to_index(suite: Suite) -> u8 {
    match suite{
        Suite::Clubs => 0,
        Suite::Diamonds => 1,
        Suite::Hearts => 2,
        Suite::Spades => 3,
    }
}


