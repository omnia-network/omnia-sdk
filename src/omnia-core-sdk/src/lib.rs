use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use utils::MAINNET_OMNIA_BACKEND_CANISTER_ID;

use ic_cdk::api::management_canister::provisional::CanisterId;
pub mod access_key;
pub mod http;
mod ledger;
pub mod signature;
pub mod utils;

thread_local! {
    pub static INIT_PARAMS_REF_CELL: RefCell<InitParams>  = RefCell::new(InitParams::default());
}

#[derive(Clone)]
pub struct InitParams {
    /// The random number generator created in the canister that imports the SDK.
    rng: StdRng,
    /// The id of the Omnia Backend canister.
    /// If empty, the SDK will use the IC Omnia Backend canister id.
    omnia_canister_id: Option<CanisterId>,
    /// The id of the Ledger canister.
    /// If empty, the SDK will use the IC Ledger canister id.
    ledger_canister_id: Option<CanisterId>,
}

impl InitParams {
    fn default() -> Self {
        Self {
            ledger_canister_id: Some(MAINNET_LEDGER_CANISTER_ID),
            omnia_canister_id: Some(MAINNET_OMNIA_BACKEND_CANISTER_ID),
            rng: SeedableRng::from_seed([0_u8; 32]),
        }
    }

    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    pub fn omnia_canister_id(&self) -> CanisterId {
        self.omnia_canister_id.expect("Omnia canister id not set")
    }

    pub fn ledger_canister_id(&self) -> CanisterId {
        self.ledger_canister_id.expect("Ledger canister id not set")
    }
}

/// Initializes the SDK with the given parameters.
///
/// It **must** be called in the [init](ic_cdk_macros::init) and [post_upgrade](ic_cdk_macros::post_upgrade) methods of the canister that imports the SDK.
pub fn init_client(init_params: InitParams) {
    INIT_PARAMS_REF_CELL.with(|params| {
        *params.borrow_mut() = init_params;
    })
}
