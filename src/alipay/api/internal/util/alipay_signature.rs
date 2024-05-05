use ring::signature;
use std::collections::HashMap;
use log::{error, warn};

use crate::common::error::alipay::signature_error::SignatureError;

pub fn rsa_check_v1(params: &mut HashMap<String, String>, public_key: String) ->  Result<(), SignatureError> {
    let sign = params.get("sign");
    warn!("origin sign: {:?}", &sign.unwrap());
    if sign.is_none() {
        error!("sign is null, params: {:?}", params);
        return Err(SignatureError::SignFieldNull);
    }
    let content = get_sign_check_content_v1(&mut params.clone());
    if content.is_none() {
        error!("content is null, params: {:?}", params);
        return Err(SignatureError::SignContentNull);
    }
    let tmp_content = content.unwrap_or_default();
    let decoded_pairs = form_urlencoded::parse(&tmp_content.as_bytes());
    let mut decoded_str = String::new();
    for (key, value) in decoded_pairs {
        decoded_str.push_str(&format!("{}={}&", key, value));
    }
    // 去除最后一个 "&"
    decoded_str.pop();

    let tmp_sign = sign.unwrap();
    let decode_sign_pairs = form_urlencoded::parse(&tmp_sign.as_bytes());
    let mut decode_sign_str = String::new();
    for (key, value) in decode_sign_pairs {
        decode_sign_str.push_str(&format!("{}={}&", key, value));
    }
    // 去除最后一个 "&"
    decode_sign_str.pop();
    let base64_decode = base64::decode(&decode_sign_str).unwrap_or_default();
    // https://docs.rs/ring/latest/ring/signature/index.html
    let verify_public_key = signature::UnparsedPublicKey::new(
        &signature::RSA_PKCS1_2048_8192_SHA256,
        public_key.as_bytes(),
    );
    warn!("legacy params, src: {}, sign: {}", decoded_str, &decode_sign_str);
    verify_public_key.verify(decoded_str.as_bytes(), &base64_decode).map_err(|_| SignatureError::BadSignature) 
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
