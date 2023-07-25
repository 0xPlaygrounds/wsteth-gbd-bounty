mod abi;
mod pb;


use substreams_ethereum::{pb::eth::v2 as eth, Event, Function};
use substreams::{
    log, hex,
    store::{DeltaProto, Deltas, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use std::str::FromStr;
use substreams::store::{DeltaBigInt, StoreAdd, StoreAddBigInt};
use substreams::scalar::BigInt;


use pb::eth::wsteth::v1::Token;
use pb::eth::wsteth::v1::Actions;
use pb::eth::wsteth::v1::Action;

use pb::eth::wsteth::v1::ActionType;
use pb::eth::wsteth::v1::Transfers;
use pb::eth::wsteth::v1::Transfer;


use abi::wsteth::functions::Wrap as WrapCall;
use abi::wsteth::functions::Unwrap as UnwrapCall;
use abi::wsteth::functions::Transfer as TransferCall;

use abi::wsteth::events::Transfer as WSTETHTransfer;


#[substreams::handlers::map]
pub fn map_actions(block: eth::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        actions: get_actions(&block).collect(),
    })
}

fn get_actions<'a>(block: &'a eth::Block) -> impl Iterator<Item = Action> + 'a {
    block
    .transactions()
    .filter_map(
        |tx| match tx {
            tx if tx.to == hex!("7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0") => {
            Some(tx)
            }
            _ => None
        }
    )
    .flat_map(
        |tx| 
        tx
        .logs_with_calls()
        .map(
            |(log, call)| {

                if let Some(_) = WrapCall::match_and_decode(call){
                    (ActionType::Wrap, call, log)
                } else if let Some(_) = UnwrapCall::match_and_decode(call) {
                    (ActionType::Unwrap, call, log)
                } else if let Some(_) = TransferCall::match_and_decode(call){
                    (ActionType::Send, call, log)
                } else {
                    (ActionType::Other, call, log)
                }
            }
        )
        .map(
            |(a, c,l)| {
                let (from, to, value) = if let Some(t) = WSTETHTransfer::match_and_decode(l) {
                    (Hex(t.from).to_string(), Hex(t.to).to_string(), t.value.to_string())
                } else {
                    (String::from(""),String::from(""), String::from(""))
                };
                
                let hash = Hex(&tx.hash).to_string();
                let b_n = &block.number.to_string();
                let timestamp = &block.timestamp_seconds().to_string();

                Action {
                    action_type: a.into(),
                    method: Hex(&c.call.input).to_string(),
                    account: Hex(&tx.from).to_string(),
                    amount: value.to_string(),
                    transfer: Some(Transfer {
                        from: from,
                        to: to,
                        amount: value.to_string(),
                        tx_hash: hash.to_string(),
                        block_number: b_n.to_string(),
                        timestamp: timestamp.to_string(),
                        log_index: l.index.to_string()
                    }),
                    tx_hash: hash,
                    block_number: b_n.to_string(),
                    timestamp: timestamp.to_string()

                }
            }
            
        )
    )
}


// #[substreams::handlers::store]
// pub fn store_token(block: eth::Block, o: StoreSetProto<Token>) {
//     if block.number == 11888477 as u64 {
//         let token = Token {
//             name: "wstETH".to_string(),
//             address: String::from("0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0"),
//             decimal: "18".to_string(),
//             symbol: "WSTETH".to_string(),
//         };
//         o.set(0, format!("Token: {}", token.address), &token);
//     };
// }

// #[substreams::handlers::store]
// pub fn store_account_holdings(txs: Transactions, o: StoreAddBigDecimal) {
//     for tx in txs.transactions {
//         o.add(
//             0,
//             format!("Account: {}", &tx.from.to_string()),
//             BigInt::from_str(tx.amount.as_str()).unwrap().neg(),
//         );

//         o.add(
//             0,
//             format!("Account: {}", &tx.to.to_string()),
//             BigInt::from_str(tx.amount.as_str()).unwrap(),
//         );
//     }

// }