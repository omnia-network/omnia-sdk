use ic_cdk::{api::{management_canister::provisional::CanisterId, print, time}, call};
use ic_ledger_types::{
    transfer, AccountIdentifier, BlockIndex, Memo, Timestamp, Tokens,
    TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT,
};
use omnia_core_sdk::access_key::AccessKeyUID;

use crate::INIT_PARAMS_REF_CELL;

pub const ACCESS_KEY_PRICE: Tokens = Tokens::from_e8s(1_000_000);

pub async fn request_access_key() -> Result<AccessKeyUID, String>{
    let params = INIT_PARAMS_REF_CELL.with(|params| {
        params.borrow().clone()
    });
    let block_index = transfer_to(
        params.ledger_canister_id.expect("ledger canister principal must be set"),
        params.omnia_canister_id.expect("omnia canister principal must be set"),
        ACCESS_KEY_PRICE
    ).await?;

    call::<(BlockIndex,), (Result<AccessKeyUID, String>,)>(
        params.omnia_canister_id.expect("omnia canister principal must be set"),
        "obtainAccessKey",
        (block_index,),
    )
    .await
    .unwrap()
    .0
}

async fn transfer_to(
    ledger_canister_id: CanisterId,
    principal: CanisterId,
    amount: Tokens,
) -> Result<BlockIndex, String> {
    let block_index = transfer(
        ledger_canister_id,
        TransferArgs {
            memo: Memo(0),
            amount,
            fee: DEFAULT_FEE,
            from_subaccount: None,
            to: AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT),
            created_at_time: Some(Timestamp {
                timestamp_nanos: time(),
            }),
        },
    )
    .await
    .map_err(|e| format!("call to ledger failed: {:?}", e))?
    .map_err(|e| format!("transfer failed: {:?}", e))?;

    print(format!(
        "Created block with index: {:?}, transferred: {:?} to principal ID: {:?}",
        block_index,
        amount,
        principal.to_string()
    ));

    Ok(block_index)
}