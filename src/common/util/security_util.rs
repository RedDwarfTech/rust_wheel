use data_encoding::HEXLOWER;
use ring::digest::{self, Algorithm};

pub fn get_sha(password: String, salt: &str) -> String {
    let calc = salt.to_owned() + &password.to_string();
    let actual = digest::digest(&digest::SHA512, calc.as_ref());
    let calc_digest_str = HEXLOWER.encode(actual.as_ref());
    return calc_digest_str;
}

pub fn get_str_sha(hash_str: String, algorithm: &'static Algorithm) -> String {
    let actual = digest::digest(algorithm, hash_str.as_ref());
    let calc_digest_str = HEXLOWER.encode(actual.as_ref());
    return calc_digest_str;
}
