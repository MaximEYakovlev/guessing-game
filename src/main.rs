mod game;
mod input;
mod random;
mod result_handler;

fn main() {
    println!("Guess the number!");

    let secret_number = random::generate_random_number(1..=100);
    let game = game::Game::new(secret_number);

    loop {
        println!("Please input your guess.");

        match input::get_guess() {
            Some(guess) => {
                println!("You guessed: {}", guess);
                let result = result_handler::handle_result(game.check_guess(guess));
                if result {
                    break;
                }
            }
            None => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
}
