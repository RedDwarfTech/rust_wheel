use ring::digest;
use data_encoding::HEXLOWER;

pub fn get_sha(password: String, salt: &str) -> String {
    let calc = password.to_string() + salt;
    let actual = digest::digest(&digest::SHA512, calc.as_ref());
    let calc_digest_str = HEXLOWER.encode(actual.as_ref());
    return calc_digest_str;
}



