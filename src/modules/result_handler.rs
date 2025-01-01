use crate::modules::output::OutputHandler;
use crate::game::GuessResult;

pub fn handle_result(output: &dyn OutputHandler, result: GuessResult) -> bool {
    match result {
        GuessResult::Correct => {
            output.print("Congratulations! You guessed correctly.");
            true
        }
        GuessResult::TooLow => {
            output.print("Too low! Try again.");
            false
        }
        GuessResult::TooHigh => {
            output.print("Too high! Try again.");
            false
        }
    }
}
