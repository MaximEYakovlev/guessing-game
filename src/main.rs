mod game;
mod input;
mod random;
mod result_handler;
mod output;

fn main() {
    output::print_welcome_message();

    let secret_number = random::generate_random_number(1..=100);
    let game = game::Game::new(secret_number);

    loop {
        output::print_prompt();

        match input::get_guess() {
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
