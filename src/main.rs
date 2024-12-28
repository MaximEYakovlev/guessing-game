mod game;
mod engine;
mod output;
mod random;
mod result_handler;

use engine::input::InputHandler;

fn main() {
    output::print_welcome_message();

    let secret_number = random::generate_random_number(1..=100);
    let game = game::Game::new(secret_number);

    let input = engine::input::ConsoleInput;

    loop {
        output::print_prompt();

        match input.read() {
            Some(guess) => {
                output::print_guess(guess);
                let result = result_handler::handle_result(game.check_guess(guess));
                if result {
                    break;
                }
            }
            None => {
                output::print_invalid_input();
                continue;
            }
        }
    }
}
