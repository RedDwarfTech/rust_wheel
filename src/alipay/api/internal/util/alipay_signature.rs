use ring::signature;
use std::collections::HashMap;
use log::error;

use crate::common::error::alipay::signature_error::SignatureError;

pub fn rsa_check_v1(params: &mut HashMap<String, String>, public_key: String) ->  Result<(), SignatureError> {
    let sign = params.get("sign");
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
    let origin_content_str = urlencoding::decode(&tmp_content);
    let origin_content = origin_content_str.unwrap_or_default().into_owned();

    let tmp_sign = sign.unwrap();
    let origin_sign_str = urlencoding::decode(&tmp_sign);
    let origin_sign = origin_sign_str.unwrap_or_default().into_owned();
    let base64_decode = base64::decode(&origin_sign).unwrap_or_default();
    // https://docs.rs/ring/latest/ring/signature/index.html
    let verify_public_key = signature::UnparsedPublicKey::new(
        &signature::RSA_PKCS1_2048_8192_SHA256,
        public_key.as_bytes(),
    );
    verify_public_key.verify(origin_content.as_bytes(), &base64_decode).map_err(|_| SignatureError::BadSignature) 
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
