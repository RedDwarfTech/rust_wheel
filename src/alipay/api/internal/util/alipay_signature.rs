use ring::signature;
use std::collections::HashMap;

use crate::common::error::alipay::signature_error::SignatureError;

/// 校验签名
/// 
/// # 参数
/// 
/// * decoded_str - 解码后的参数
/// * decoded_sign - 解码后的signature
/// * public_key - 公钥
/// 
/// 返回校验的结果
/// 
/// 
pub fn rd_rsa_check_v1(
    decoded_str: &String,
    decoded_sign: &String,
    public_key: String,
) -> Result<(), SignatureError> {
    // https://docs.rs/ring/latest/ring/signature/index.html
    let verify_public_key = signature::UnparsedPublicKey::new(
        &signature::RSA_PKCS1_2048_8192_SHA256,
        public_key.as_bytes(),
    );
    verify_public_key
        .verify(decoded_str.as_bytes(), &decoded_sign.as_bytes())
        .map_err(|_| SignatureError::BadSignature)
}

pub fn get_sign_check_content_v1(params: &mut HashMap<String, String>) -> Option<String> {
    if params.is_empty() {
        return None;
    }
    params.remove("sign");
    params.remove("sign_type");

    let mut keys: Vec<_> = params.keys().cloned().collect();
    keys.sort();

    let mut content = String::new();
    for (index, key) in keys.iter().enumerate() {
        if let Some(value) = params.get(key) {
            if !key.is_empty() && !value.is_empty() {
                content.push_str(if index == 0 { "" } else { "&" });
                content.push_str(&format!("{}={}", key, value));
            }
        }
    }

    Some(content)
}
