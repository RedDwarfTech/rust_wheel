use rand::distr::Alphanumeric;
use rand::rng;
use rand::RngExt;

pub fn generate_random_string(length: usize) -> String {
    let rng = rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}