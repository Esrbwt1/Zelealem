use ring::{digest, rand::{self, SecureRandom}};
use serde::Serialize; // Added for PublicKey serialization

pub type Hash = [u8; 32];
pub type Signature = [u8; 64];

// CORRECTED: PublicKey is now a struct that can derive traits.
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct PublicKey(pub [u8; 32]);

pub fn hash_data(data: &[u8]) -> Hash {
    let mut context = digest::Context::new(&digest::SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().try_into().expect("SHA-256 should always produce 32 bytes")
}

pub fn generate_keypair() -> (PublicKey, Vec<u8>) {
    let rng = rand::SystemRandom::new();
    let mut secret_key = vec![0u8; 32];
    rng.fill(&mut secret_key).expect("Failed to generate random data for secret key");
    // CORRECTED: Wrap the hash in the PublicKey struct.
    let public_key = PublicKey(hash_data(&secret_key));
    (public_key, secret_key)
}

pub fn sign_data(data: &[u8], secret_key: &[u8]) -> Signature {
    let mut signature = [0u8; 64];

    let mut content_to_sign = Vec::from(data);
    content_to_sign.extend_from_slice(secret_key);
    let proof_hash = hash_data(&content_to_sign);
    signature[..32].copy_from_slice(&proof_hash);

    // CORRECTED: Wrap the hash in PublicKey struct before accessing its inner array.
    let signer_pub_key = PublicKey(hash_data(secret_key));
    signature[32..].copy_from_slice(&signer_pub_key.0);

    signature
}

pub fn verify_signature(signature: &Signature, _data: &[u8], owner_public_key: &PublicKey) -> bool {
    let signer_public_key: &[u8] = &signature[32..];
    // CORRECTED: Access the inner array of the owner's key with .0
    if signer_public_key != &owner_public_key.0 {
        return false;
    }
    true
}