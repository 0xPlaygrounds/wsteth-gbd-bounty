mod abi;
mod pb;


use abi::wsteth;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams::{
    log, hex,
    store::{DeltaProto, Deltas, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use std::str::FromStr;
use substreams::store::{DeltaBigInt, StoreAdd, StoreAddBigInt};
use substreams::scalar::BigInt;

use pb::eth::wsteth::v1::Transactions;
use pb::eth::wsteth::v1::Transaction;
use pb::eth::wsteth::v1::Token;
use pb::eth::wsteth::v1::Account;

use abi::wsteth::functions::Wrap as WrapCall;
use abi::wsteth::functions::Unwrap as UnwrapCall;
use abi::wsteth::events::Transfer as WSTETHTransfer;


#[substreams::handlers::map]
pub fn map_transactions(block: eth::Block) -> Result<Transactions, substreams::errors::Error> {
    Ok(Transactions {
        transactions: get_transactions(&block).collect(),
    })
}

fn get_transactions<'a>(block: &'a eth::Block) -> impl Iterator<Item = Transaction> + 'a {
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
    .map(
        |tx| Transaction {
            from: Hex(&tx.from).to_string(),
            to: Hex(&tx.to).to_string(),
            amount: String::from("0"),
            tx_hash: Hex(&tx.hash).to_string(),
        }
    )
}

#[substreams::handlers::store]
pub fn store_token(block: eth::Block, o: StoreSetProto<Token>) {
    if block.number == 11888477 as u64 {
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
pub fn store_account_holdings(txs: Transactions, o: StoreAddBigInt) {
    for tx in txs.transactions {
        o.add(
            0,
            format!("Account: {}", &tx.from.to_string()),
            BigInt::from_str(tx.amount.as_str()).unwrap().neg(),
        );

        o.add(
            0,
            format!("Account: {}", &tx.to.to_string()),
            BigInt::from_str(tx.amount.as_str()).unwrap(),
        );
    }

}