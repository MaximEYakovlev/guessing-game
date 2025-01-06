use rand::Rng;
use std::ops::RangeInclusive;

pub trait RandomNumber {
    fn generate_random_number(range: RangeInclusive<u32>) -> u32;
}

pub fn generate_random_number(range: RangeInclusive<u32>) -> u32 {
    rand::thread_rng().gen_range(range)
}

#[cfg(test)]
mod tests {
    use super::generate_random_number;

    #[test]
    fn test_generate_random_number_within_range() {
        let range = 1..=10;
        for _ in 0..100 {
            let result = generate_random_number(range.clone());
            assert!(range.contains(&result), "Result {} is not within range {:?}", result, range);
        }
    }
}
