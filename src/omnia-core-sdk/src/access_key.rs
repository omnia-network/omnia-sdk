use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::Sha256;

pub type AccessKeyUID = String;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UniqueAccessKey {
    nonce: u128,
    key: AccessKeyUID,
}

impl UniqueAccessKey {
    pub fn new(nonce: u128, key: AccessKeyUID) -> Self {
        Self { nonce, key }
    }

    pub fn get_nonce(&self) -> u128 {
        self.nonce
    }

    pub fn get_key(&self) -> AccessKeyUID {
        self.key.clone()
    }

    /// Serialize the UniqueAccessKey to a string
    pub fn serialize(&self) -> String {
        to_string(self).expect("UniqueAccessKey serialization failed")
    }

    /// Generate a **sha256** hash of the UniqueAccessKey
    pub fn generate_hash(&self) -> [u8; 32] {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(self.serialize().as_bytes());
        hasher.finalize().into()
    }
}
