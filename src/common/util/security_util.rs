use data_encoding::HEXLOWER;
use hmac::{Hmac, Mac};
use ring::digest::{self, Algorithm};
use sha2::Sha256;

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

pub fn generate_signature(params: &[(String, String)], secret: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    let data = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let signature = result.into_bytes();
    hex::encode(signature)
}

pub fn verify_signature(params: &[(String, String)], secret: &str, signature: &str) -> bool {
    let expected_signature = generate_signature(params, secret);
    expected_signature == signature
}