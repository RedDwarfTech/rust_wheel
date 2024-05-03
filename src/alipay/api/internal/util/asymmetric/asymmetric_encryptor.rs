use std::error::Error;

pub trait AsymmetricEncryptor {
    fn sign(&self, content: &str, charset: &str, private_key: &str) -> Result<String, Box<dyn Error>>;
    fn verify(&self, content: &str, charset: &str, public_key: &str, sign: &str) -> Result<bool, Box<dyn Error>>;
    fn encrypt(&self, plain_text: &str, charset: &str, public_key: &str) -> Result<String, Box<dyn Error>>;
    fn decrypt(&self, cipher_text_base64: &str, charset: &str, private_key: &str) -> Result<String, Box<dyn Error>>;
}
