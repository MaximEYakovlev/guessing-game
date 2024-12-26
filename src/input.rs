use std::io;

pub fn get_guess() -> Option<u32> {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
