use candid::Principal;
use ic_cdk::api::management_canister::provisional::CanisterId;

use crate::INIT_PARAMS_REF_CELL;

/// The actual Omnia Backend canister id on the IC: `zaqyp-oyaaa-aaaap-qbh4a-cai`
pub const MAINNET_OMNIA_BACKEND_CANISTER_ID: CanisterId =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x01, 0xf0, 0x09, 0xf8, 0x01, 0x01]);

/// Based on the Omnia Backend canister id in the initial parameters,
/// returns whether the current environment is the mainnet or not.
///
/// ### Warning!
/// If deployed on the local replica without setting the Omnia Backend canister id in the initial parameters,
/// this function will return **true**.
pub fn is_mainnet() -> bool {
    INIT_PARAMS_REF_CELL
        .with(|params| params.borrow().omnia_canister_id() == MAINNET_OMNIA_BACKEND_CANISTER_ID)
}

/// Returns the Omnia Backend canister id based on the current environment.
///
/// ### Warning!
/// If deployed on the local replica without setting the Omnia Backend canister id in the initial parameters,
/// this function will return the mainnet Omnia Backend canister id.
pub fn get_omnia_backend_canister_id() -> CanisterId {
    INIT_PARAMS_REF_CELL.with(|params| params.borrow().omnia_canister_id())
}
