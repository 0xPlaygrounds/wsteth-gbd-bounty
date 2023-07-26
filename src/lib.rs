mod abi;
mod pb;


use substreams_ethereum::{pb::eth::v2 as eth, Event, Function};
use substreams::{
    log, hex,
    store::{DeltaProto, Deltas, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use substreams::scalar::BigDecimal;
use std::str::FromStr;
use substreams::store::{DeltaBigDecimal, StoreAdd, StoreAddBigDecimal};
use substreams::pb::substreams::store_delta::Operation;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams::errors::Error;

use pb::eth::wsteth::v1::Token;
use pb::eth::wsteth::v1::Actions;
use pb::eth::wsteth::v1::Action;

use pb::eth::wsteth::v1::ActionType;
use pb::eth::wsteth::v1::Transfer;


use abi::wsteth::functions::Wrap as WrapCall;
use abi::wsteth::functions::Unwrap as UnwrapCall;
use abi::wsteth::functions::Transfer as TransferCall;
use abi::wsteth::functions::Approve as ApproveCall;

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

                if let Some(_) = WrapCall::match_and_decode(call) {
                    (ActionType::Wrap, call, log)
                } else if let Some(_) = UnwrapCall::match_and_decode(call) {
                    (ActionType::Unwrap, call, log)
                } else if let Some(_) = TransferCall::match_and_decode(call){
                    (ActionType::Send, call, log)
                } else if let Some(_) = ApproveCall::match_and_decode(call){
                    (ActionType::Approve, call, log)
                } else {
                    // Edge case => going directly to wsteth from eth
                    // recipient is wsteth but no method, forwards call opcode directly to steth
                    // receive() external payable -> fallback that only receives eth and no call data
                    if call.call.input.is_empty() {
                        (ActionType::Wrap, call, log)
                    } else {
                        (ActionType::Other, call, log)
                    }
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


#[substreams::handlers::store]
pub fn store_token(block: eth::Block, o: StoreSetProto<Token>) {
    if block.number == 11888477 as u64 {
        log::info!("token stored");
        let token = Token {
            name: "wstETH".to_string(),
            address: String::from("0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0"),
            decimal: "18".to_string(),
            symbol: "WSTETH".to_string(),
        };
        o.set(0, format!("Token: {}", token.address), &token);
    };
}

#[substreams::handlers::store]
pub fn store_account_holdings(actions: Actions, o: StoreAddBigDecimal) {
    for action in actions.actions {
        if let Some(transfer) = action.transfer {
            let from = &transfer.from;
            let to = &transfer.to;
            let amount = match bigdecimal::BigDecimal::from_str(&transfer.amount.as_str()) {
                Ok(d) => substreams::scalar::BigDecimal::from(d),
                _ => BigDecimal::from(substreams::scalar::BigDecimal::zero())
            };

            log::info!("token transfer");

            o.add(
                0,
                format!("account: {from}"),
                amount.neg()
            );
    
            o.add(
                0,
                format!("account: {to}"),
                amount
            );
        }
    }

}


#[substreams::handlers::map]
pub fn graph_out(
    actions: Actions,
    account_holdings: Deltas<DeltaBigDecimal>,
    tokens: Deltas<DeltaProto<Token>>,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    for delta in account_holdings.deltas {
        let address = delta.key.as_str();

        match delta.operation {
            Operation::Create => {
                let row = tables.create_row("Account", address);

                row.set("holdings", delta.old_value);
            }
            Operation::Update => {
                let row = tables.update_row("Account", address);
                row.set("holdings", delta.new_value);
            }
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
    }

    for action in &actions.actions {
        match action.action_type {
            0 | 1 | 2 => {

                if let Some(transfer) = &action.transfer {
                    let id: String = format!("{}-{}",&transfer.tx_hash,&transfer.log_index);
                    let row = tables.create_row("Transfer", &id);

                    row.set("sender", &transfer.from);
                    row.set("receiver", &transfer.to);
                    row.set("token",String::from("0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0"));
                    row.set("timestamp", &transfer.timestamp);
                    row.set("blockNumber", &transfer.block_number);
                    row.set("logIndex", &transfer.log_index);
                    row.set("txHash", &transfer.tx_hash);
                    row.set("amount", &transfer.amount);
                }

                fn action_to_string(a:i32) -> String {
                    match a {
                        0 => String::from("WRAP"),
                        1 => String::from("UNWRAP"),
                        2 => String::from("SEND"),
                        _ => String::from("Other")
                    }
                }

                let aid = format!("{}-{}", action_to_string(action.action_type), action.tx_hash);
                let a_row = tables.create_row("Action", &aid);

                let tid = if let Some(t) = &action.transfer {
                    format!("{}-{}",&t.tx_hash,&t.log_index)
                } else {
                    String::from("")
                };

                a_row.set("tx_hash", &action.tx_hash);
                a_row.set("timestamp", &action.timestamp);
                a_row.set("block_number", &action.block_number);
                a_row.set("action_type", action_to_string(action.action_type));
                a_row.set("account", &action.account);
                a_row.set("token", String::from("0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0"));
                a_row.set("amount", &action.amount);
                a_row.set("transfer", format!("{tid}"));

            } 
            _ => {
                ()
            }
        };
    }

    for delta in tokens.deltas {
        match delta.operation {
            Operation::Create => {
                let token_row = tables.create_row("Token", &delta.new_value.address);
                token_row.set("name", delta.new_value.name);
                token_row.set("address", delta.new_value.address);
                token_row.set("decimals", delta.new_value.decimal);
                token_row.set("symbol", delta.new_value.symbol);
            }
            Operation::Update => todo!(),
            Operation::Delete => todo!(),
            x => panic!("unsupported opeation {:?}", x),
        };
    }

    let entity_changes = tables.to_entity_changes();
    Ok(entity_changes)
}