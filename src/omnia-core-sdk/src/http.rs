use ic_cdk::api::management_canister::http_request::HttpHeader;

use crate::access_key::{generate_signed_unique_access_key, AccessKeyUID};

/// Returns the headers to be used in the HTTP requests to the device URL.
///
/// * `access_key` - the access key obtained with the [request_access_key](crate::access_key::request_access_key) method.
/// * `device_headers` - (optional) the headers returned by the query to the RDF database to get the device URL.
pub async fn get_request_headers(
    access_key: AccessKeyUID,
    device_headers: Option<Vec<HttpHeader>>,
) -> Result<Vec<HttpHeader>, String> {
    let singed_access_key = generate_signed_unique_access_key(access_key).await?;

    let mut headers = device_headers.unwrap_or(vec![]);

    headers.extend(vec![
        // access key headers
        HttpHeader {
            name: "X-Omnia-Access-Key".to_string(),
            value: singed_access_key.unique_access_key.get_key(),
        },
        HttpHeader {
            name: "X-Omnia-Access-Key-Nonce".to_string(),
            value: singed_access_key.unique_access_key.get_nonce().to_string(),
        },
        HttpHeader {
            name: "X-Omnia-Access-Key-Signature".to_string(),
            value: singed_access_key.signature_hex,
        },
        // additional headers
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ]);

    Ok(headers)
}
