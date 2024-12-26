use rand::Rng;

pub fn generate_random_number(range: std::ops::RangeInclusive<u32>) -> u32 {
    rand::thread_rng().gen_range(range)
}
