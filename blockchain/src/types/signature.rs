use ed25519_dalek::{Signer, Verifier};

use crate::types::Hash;

pub struct HelloSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub signing_key: S,
}

pub struct HelloVerifier<V> {
    pub verify_key: V,
}

impl<S> HelloSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub fn sign(&self, hash: Hash) -> ed25519::Signature {
        self.signing_key.sign(hash.as_bytes())
    }
}

impl<V> HelloVerifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    pub fn verify(&self, hash: Hash, signature: &ed25519::Signature) -> Result<(), ed25519::Error> {
        self.verify_key.verify(hash.as_bytes(), signature)
    }
}
