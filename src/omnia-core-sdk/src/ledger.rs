use candid::Principal;
use ic_cdk::api::{print, time};
use ic_ledger_types::{
    transfer, AccountIdentifier, BlockIndex, Memo, Timestamp, Tokens, TransferArgs, DEFAULT_FEE,
    DEFAULT_SUBACCOUNT,
};

use crate::INIT_PARAMS_REF_CELL;

pub async fn transfer_to(principal: Principal, amount: Tokens) -> Result<BlockIndex, String> {
    let ledger_canister_id =
        INIT_PARAMS_REF_CELL.with(|params| params.borrow().ledger_canister_id());
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
