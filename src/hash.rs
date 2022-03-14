use hmac::Hmac;
use hmac::Mac;
use sha2::Digest;
use sha2::Sha256;

#[allow(dead_code)]
pub fn sha256(content: &[u8]) -> Vec<u8> {
    Sha256::digest(content).as_slice().to_vec()
}

pub fn hex_sha256(content: &[u8]) -> String {
    hex::encode(Sha256::digest(content).as_slice())
}

pub fn hmac_sha256(key: &[u8], content: &[u8]) -> Vec<u8> {
    let mut h = Hmac::<Sha256>::new_from_slice(key).expect("invalid key length");
    h.update(content);

    h.finalize().into_bytes().to_vec()
}

pub fn hex_hmac_sha256(key: &[u8], content: &[u8]) -> String {
    let mut h = Hmac::<Sha256>::new_from_slice(key).expect("invalid key length");
    h.update(content);

    hex::encode(h.finalize().into_bytes())
}