pub trait OutputHandler {
    fn print_welcome_message(&self);
    fn print_prompt(&self);
    fn print_guess(&self, guess: u32);
    fn print_invalid_input(&self);
}

pub struct ConsoleOutput;

impl OutputHandler for ConsoleOutput {
    fn print_welcome_message(&self) {
        println!("Welcome to the Guessing Game!");
    }

    fn print_prompt(&self) {
        println!("Please input your guess.");
    }

    fn print_guess(&self, guess: u32) {
        println!("You guessed: {guess}");
    }

    fn print_invalid_input(&self) {
        println!("Invalid input, please try again.");
    }
}
