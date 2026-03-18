pub(crate) mod aes;

use aes_gcm::aead::OsRng;
use anyhow::Result;
use p256::elliptic_curve::ScalarPrimitive;

pub(crate) fn generate_p256_secret() -> Result<Vec<u8>> {
    let sk = p256::SecretKey::new(ScalarPrimitive::random(&mut OsRng));
    let sk = sk.to_bytes();
    Ok(sk.to_vec())
}

pub(crate) fn get_p256_pubkey(sk: &[u8]) -> Vec<u8> {
    let sk = p256::SecretKey::from_slice(sk).unwrap();
    let pubkey = sk.public_key();
    pubkey.to_sec1_bytes().to_vec()
}
