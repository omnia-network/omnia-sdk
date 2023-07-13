use candid::CandidType;
use ic_cdk::{
    api::management_canister::ecdsa::{
        sign_with_ecdsa, SignWithEcdsaArgument, SignWithEcdsaResponse,
    },
    call,
};
use ic_ledger_types::{BlockIndex, Tokens};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::Sha256;

use crate::{
    ledger::transfer_to,
    random::generate_nonce,
    signature::{get_ecdsa_key_id, SignatureReply},
    INIT_PARAMS_REF_CELL,
};

pub const ACCESS_KEY_PRICE: Tokens = Tokens::from_e8s(100_000);

pub type AccessKeyUID = String;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UniqueAccessKey {
    nonce: u128,
    key: AccessKeyUID,
}

impl UniqueAccessKey {
    pub fn new(key: AccessKeyUID) -> Self {
        let nonce = generate_nonce();
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

    pub async fn generate_signature(&self) -> Result<SignWithEcdsaResponse, String> {
        let request = SignWithEcdsaArgument {
            message_hash: self.generate_hash().to_vec(),
            derivation_path: vec![],
            key_id: get_ecdsa_key_id(),
        };

        let (response,) = sign_with_ecdsa(request)
            .await
            .map_err(|e| format!("sign_with_ecdsa failed {:?}", e))?;
        Ok(response)
    }
}

pub async fn generate_signed_unique_access_key(
    access_key: AccessKeyUID,
) -> Result<SignatureReply, String> {
    let unique_access_key = UniqueAccessKey::new(access_key);

    Ok(SignatureReply {
        signature_hex: hex::encode(unique_access_key.generate_signature().await?.signature),
        unique_access_key,
    })
}

pub async fn request_access_key() -> Result<AccessKeyUID, String> {
    let omnia_canister_id = INIT_PARAMS_REF_CELL.with(|params| params.borrow().omnia_canister_id());
    let block_index = transfer_to(omnia_canister_id, ACCESS_KEY_PRICE).await?;

    // TODO: delay the call to Omnia Backend in order to let the IC finalize the transfer for the fee

    call::<(BlockIndex,), (Result<AccessKeyUID, String>,)>(
        omnia_canister_id,
        "obtainAccessKey",
        (block_index,),
    )
    .await
    .unwrap()
    .0
}
