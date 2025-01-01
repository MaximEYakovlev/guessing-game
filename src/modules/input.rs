use std::io;

pub mod input {
    pub trait InputHandler {
        fn read(&self) -> Option<u32>;
    }

    pub struct ConsoleInput;

    impl InputHandler for ConsoleInput {
        fn read(&self) -> Option<u32> {
            let mut input = String::new();

            io::stdin().read_line(&mut input).ok()?;

            match input.trim().parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        }
    }
}
