#[derive(Debug)]
pub enum SignatureError {
   BadPrivateKey,
   OOM,
   BadSignature,
   SignFieldNull,
   SignContentNull
}