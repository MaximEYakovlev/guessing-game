use std::cmp::Ordering;

pub struct Game {
    pub secret_number: u32,
}

impl Game {
    pub fn new(secret_number: u32) -> Self {
        Game { secret_number }
    }

    pub fn check_guess(&self, guess: u32) -> Ordering {
        guess.cmp(&self.secret_number)
    }
}
