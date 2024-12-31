pub enum GuessResult {
    Correct,
    TooLow,
    TooHigh,
}

pub struct Game {
    pub secret_number: u32,
}

impl Game {
    pub fn new(secret_number: u32) -> Self {
        Game { secret_number }
    }

    pub fn check_guess(&self, guess: u32) -> GuessResult {
        match guess.cmp(&self.secret_number) {
            std::cmp::Ordering::Equal => GuessResult::Correct,
            std::cmp::Ordering::Less => GuessResult::TooLow,
            std::cmp::Ordering::Greater => GuessResult::TooHigh,
        }
    }
}
