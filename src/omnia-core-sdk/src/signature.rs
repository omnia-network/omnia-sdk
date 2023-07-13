use candid::CandidType;
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId};
use serde::Serialize;

use crate::{access_key::UniqueAccessKey, utils::is_mainnet};

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureReply {
    pub signature_hex: String,
    pub unique_access_key: UniqueAccessKey,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureVerificationReply {
    pub is_signature_valid: bool,
}

pub enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl EcdsaKeyIds {
    pub fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

/// Returns the ECDSA key id based on the environment.
pub fn get_ecdsa_key_id() -> EcdsaKeyId {
    if is_mainnet() {
        EcdsaKeyIds::TestKey1.to_key_id()
    } else {
        EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id()
    }
}
