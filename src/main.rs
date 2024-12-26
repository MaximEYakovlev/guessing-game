mod game;
mod input;
mod random;

fn main() {
    println!("Guess the number!");

    let secret_number = random::generate_random_number(1..=100);
    let game = game::Game::new(secret_number);

    loop {
        println!("Please input your guess.");

        match input::get_guess() {
            Some(guess) => {
                println!("You guessed: {}", guess);
                match game.check_guess(guess) {
                    std::cmp::Ordering::Less => println!("Too small!"),
                    std::cmp::Ordering::Greater => println!("Too big!"),
                    std::cmp::Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
            None => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
}
