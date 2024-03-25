use std::str::FromStr;

use anyhow::{Context, Result};
use revm::{
    db::EmptyDB,
    primitives::{AccountInfo, Address, BlockEnv, Bytes, ExecutionResult, TransactTo, TxEnv, U256},
    Evm, InMemoryDB,
};

fn main() {
    let mut evm = create_evm();

    println!("evm spec id: {:?}", evm.spec_id());

    let address_1 = Address::from_str("0x67b1d87101671b127f5f8714789C7192f7ad340e").unwrap();
    let address_2 = Address::from_str("0xa94f5374Fce5edBC8E2a8697C15331677e6EbF0B").unwrap();

    // need to give an initial balance to the account
    set_account_balance(evm.db_mut(), &address_1, 220_000);

    println!(
        "initial balance account 1: {}",
        get_account_balance(evm.db_mut(), &address_1)
    );
    println!(
        "initial balance account 2: {}",
        get_account_balance(evm.db_mut(), &address_2)
    );
    println!("initial block: {:?}", evm.block());

    // This is an example tx.
    // Later, we can have a vec of TxEnv to process at once as a "block", increment db block
    // num, then return the new db state
    let example_transaction = TxEnv {
        caller: address_1,
        gas_limit: 21_000,
        gas_price: U256::from(10),
        transact_to: TransactTo::Call(address_2),
        value: U256::from(1_000),
        data: Bytes::default(),
        nonce: Some(0),
        // the chain id of out evm is 1 by default
        chain_id: Some(1),
        // idk
        access_list: vec![],
        // idc
        gas_priority_fee: None,
        // idc about blobs
        blob_hashes: vec![],
        max_fee_per_blob_gas: None,
    };

    let tx_result = process_tx(&mut evm, example_transaction).unwrap();

    // at this point the transaction has update the db state, so we can poke around and look
    // at new values. etc

    // look at this cool and useful info
    match tx_result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            logs,
            output,
        } => {
            println!(
                "Success!  reason: {:?}, gas_used: {}, gas_refunded: {}, logs: {:?}, output: {:?}",
                reason, gas_used, gas_refunded, logs, output
            );
        }
        ExecutionResult::Halt { reason, gas_used } => {
            println!("Halted, reason: {:?}, gas_used: {}", reason, gas_used);
        }
        ExecutionResult::Revert { gas_used, output } => {
            println!("Reverted, gas_used: {:?}, output: {:?}", gas_used, output);
        }
    }

    increment_block(&mut evm);

    println!(
        "ending balance account 1: {}",
        get_account_balance(evm.db_mut(), &address_1)
    );
    println!(
        "ending balance account 2: {}",
        get_account_balance(evm.db_mut(), &address_2)
    );
    println!("ending block: {:?}", evm.block());
}

// static lifetime is okay for this example because we want evm to live the entire duration
// of our program (execution of main)
fn create_evm() -> Evm<'static, (), InMemoryDB> {
    // use a db type that holds all state changes in memory
    let db = InMemoryDB::new(EmptyDB::new());
    // create our evm instance with this db
    let evm_builder = Evm::builder().with_db(db);
    evm_builder.build()
}

fn process_tx(evm: &mut Evm<'_, (), InMemoryDB>, tx: TxEnv) -> Result<ExecutionResult> {
    *evm.tx_mut() = tx;

    evm.transact_commit().context("commit transaction to db")
}

// updates out in-memory db to have a balance associated to this address
// needed for initial block
fn set_account_balance(db: &mut InMemoryDB, address: &Address, balance: u64) {
    let balance = U256::from(balance);
    let account_info = AccountInfo::from_balance(balance);

    db.insert_account_info(*address, account_info);
}

// helper for getting an address's balance
fn get_account_balance(db: &mut InMemoryDB, address: &Address) -> U256 {
    db.load_account(*address).unwrap().info.balance
}

// For future!
// mutates current evm state in place
fn increment_block<'a>(evm: &mut Evm<'_, (), InMemoryDB>) {
    let old_block = evm.block();

    // change whatever fields you want
    let new_block = BlockEnv {
        number: old_block.number + U256::from(1),
        // have to clone because not all fields implement copy.
        // This clone is probably fine
        ..old_block.clone()
    };

    // set the evm's block to the new block
    *evm.block_mut() = new_block;
}
