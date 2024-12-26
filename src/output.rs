pub fn print_welcome_message() {
    println!("Guess the number!");
}

pub fn print_prompt() {
    println!("Please input your guess.");
}

pub fn print_invalid_input() {
    println!("Invalid input. Please enter a number.");
}

pub fn print_guess(guess: u32) {
    println!("You guessed: {}", guess);
}
