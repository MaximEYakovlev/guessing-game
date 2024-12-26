use std::cmp::Ordering;

pub fn handle_result(ordering: Ordering) -> bool {
    match ordering {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
