use omnia_core_sdk::{access_key::UniqueAccessKey, signature::SignatureReply};

pub async fn gnerate_signed_unique_access_key(unique_access_key: UniqueAccessKey) -> Result<SignatureReply, String> {
    Ok(SignatureReply {
        signature_hex: hex::encode(unique_access_key.generate_signature().await?.signature),
        unique_access_key,
    })
}
