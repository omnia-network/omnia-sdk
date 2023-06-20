use std::cell::RefCell;

use ic_cdk::api::management_canister::provisional::CanisterId;
pub use omnia_core_sdk as core;
pub mod access_key;
pub mod ledger;

thread_local! {
    pub static INIT_PARAMS_REF_CELL: std::cell::RefCell<InitParams>  = RefCell::new(InitParams::default());
}

#[derive(Clone)]
pub struct InitParams {
    ledger_canister_id: Option<CanisterId>,
    omnia_canister_id: Option<CanisterId>,
}

impl InitParams {
    fn default() -> Self {
        Self {
            ledger_canister_id: None,
            omnia_canister_id: None,
        }
    }
}

pub fn init_client(ledger_canister_id: CanisterId, omnia_canister_id: CanisterId) {
    INIT_PARAMS_REF_CELL.with(|params| {
        params.borrow_mut().ledger_canister_id = Some(ledger_canister_id);
        params.borrow_mut().omnia_canister_id = Some(omnia_canister_id);
    })
}