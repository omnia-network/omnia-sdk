use ic_cdk::api::management_canister::http_request::HttpHeader;
use rand::Rng;

use crate::{
    access_key::{generate_signed_unique_access_key, AccessKeyUID},
    INIT_PARAMS_REF_CELL,
};

/// Returns the headers to be used in the HTTP requests to the device URL.
/// It includes the access key headers and the idempotency header, along with some additional headers.
///
/// * `access_key` - the access key obtained with the [request_access_key](crate::access_key::request_access_key) method.
/// * `device_headers` - (optional) the headers returned by the query to the RDF database to get the device URL.
pub async fn get_request_headers(
    access_key: AccessKeyUID,
    device_headers: Option<Vec<HttpHeader>>,
) -> Result<Vec<HttpHeader>, String> {
    let singed_access_key = generate_signed_unique_access_key(access_key).await?;

    let idempotent_key: u64 = INIT_PARAMS_REF_CELL
        .with(|params| params.borrow_mut().rng().clone())
        .gen();

    let mut headers = device_headers.unwrap_or(vec![]);

    headers.extend(vec![
        // access key headers
        HttpHeader {
            name: String::from("X-Omnia-Access-Key"),
            value: singed_access_key.unique_access_key.get_key(),
        },
        HttpHeader {
            name: String::from("X-Omnia-Access-Key-Nonce"),
            value: singed_access_key.unique_access_key.get_nonce().to_string(),
        },
        HttpHeader {
            name: String::from("X-Omnia-Access-Key-Signature"),
            value: singed_access_key.signature_hex,
        },
        // idempotency header
        HttpHeader {
            name: String::from("Idempotent-Key"),
            value: idempotent_key.to_string(),
        },
        // additional headers
        HttpHeader {
            name: String::from("Content-Type"),
            value: String::from("application/json"),
        },
    ]);

    Ok(headers)
}
