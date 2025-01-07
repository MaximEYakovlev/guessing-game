mod game;
mod modules;
mod random;

use modules::input::input::InputHandler;
use modules::output::output::OutputHandler;
use modules::result_handler;
use random::{ RandomGenerator, RandomNumber };

fn main() {
    let input: &dyn InputHandler = &modules::input::input::ConsoleInput;
    let output: &dyn OutputHandler = &modules::output::output::ConsoleOutput;
    let generator: &dyn RandomNumber = &RandomGenerator;
    let secret_number = generator.generate_random_number(1..=100);
    let game = game::Game::new(secret_number);

    output.print_welcome_message();

    loop {
        output.print_prompt();

        match input.read() {
            Some(guess) => {
                output.print_guess(guess);

                let is_correct = result_handler::result_handler::handle_result(
                    output,
                    game.check_guess(guess)
                );

                if is_correct {
                    break;
                }
            }
            None => {
                output.print_invalid_input();
                continue;
            }
        }
    }
}
