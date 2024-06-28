use std::path::PathBuf;

use divan::{black_box, Bencher};
use evm_bench::{revmc_runner::build_evm, rust_evm_runner::run_in_rust_evm};
use evm_mlir::{
  context::Context as emContext, db::Db, executor::Executor, primitives::Bytes, program::Program,
  syscall::SyscallContext, Env,
};
use revm::{
  db::{BenchmarkDB, CacheDB, EmptyDB},
  interpreter,
  primitives::{address, AccountInfo, Bytecode, TransactTo, U256 as rU256},
  Evm,
};

include!("./common.rs");

#[divan::bench(sample_count = 100)]
fn bench_rust_evm() {
  run_in_rust_evm(FIBONACCI_CODE, 100);
}

#[divan::bench(sample_count = 100)]
fn bench_revm_not_analyse() {
  let bytes: Vec<u8> = FIBONACCI_CODE.into();
  let raw = black_box(Bytecode::new_raw(bytes.into()));
  let bytecode = black_box(interpreter::analysis::to_analysed(raw));
  let num = rU256::from(100);
  let actual_num = num.saturating_sub(rU256::from(1));
  let calldata = actual_num.to_be_bytes_vec();
  let mut evm = black_box(
    Evm::builder()
      .with_db(BenchmarkDB::new_bytecode(bytecode))
      .modify_tx_env(|tx| {
        tx.caller = address!("1000000000000000000000000000000000000000");
        tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000000"));
        tx.data = calldata.into();
      })
      .build(),
  );

  black_box(evm.transact().unwrap());
}

#[divan::bench(sample_count = 100)]
fn bench_revm_analysed(bencher: Bencher) {
  let bytes: Vec<u8> = FIBONACCI_CODE.into();
  let raw = black_box(Bytecode::new_raw(bytes.into()));
  let bytecode = black_box(interpreter::analysis::to_analysed(raw));
  let num = rU256::from(100);
  let actual_num = num.saturating_sub(rU256::from(1));
  let calldata = actual_num.to_be_bytes_vec();
  bencher.bench_local(move || {
    let mut evm = black_box(
      Evm::builder()
        .with_db(BenchmarkDB::new_bytecode(bytecode.clone()))
        .modify_tx_env(|tx| {
          tx.caller = address!("1000000000000000000000000000000000000000");
          tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000000"));
          tx.data = calldata.clone().into(); // shared_calldata.clone().into();
        })
        .build(),
    );

    black_box(evm.transact().unwrap());
  });
}

#[divan::bench(sample_count = 100)]
fn bench_evm_mlir(bencher: Bencher) {
  let bytes: Vec<u8> = FIBONACCI_CODE.into();
  let program = Program::from_bytecode(&bytes).unwrap();
  let output_file = PathBuf::from("output");
  let context = emContext::new();
  let module = context.compile(&program, &output_file).expect("failed to compile program");
  let executor = Executor::new(&module, Default::default());
  let mut env: Env = Default::default();
  env.tx.gas_limit = 999_999;
  let num = rU256::from(100);
  let actual_num = num.saturating_sub(rU256::from(1));
  let calldata = actual_num.to_be_bytes_vec();
  env.tx.data = Bytes::from(calldata);
  let mut db = Db::default();
  let mut context = SyscallContext::new(env, &mut db);
  let initial_gas = 999_999_999;
  bencher.bench_local(move || {
    executor.execute(black_box(&mut context), black_box(initial_gas));
    assert!(context.get_result().unwrap().result.is_success());
  });
}

#[divan::bench(sample_count = 100)]
fn bench_revmc(bencher: Bencher) {
  let num = rU256::from(100);
  // The bytecode runs fib(input + 1), so we need to subtract 1.
  let actual_num = num.saturating_sub(rU256::from(1));

  let db = CacheDB::new(EmptyDB::new());
  let mut evm = build_evm(db);
  let fibonacci_address = address!("0000000000000000000000000000000000001234");
  evm.db_mut().insert_account_info(
    fibonacci_address,
    AccountInfo {
      code_hash: FIBONACCI_HASH.into(),
      code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
      ..Default::default()
    },
  );
  evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
  evm.context.evm.env.tx.data = actual_num.to_be_bytes_vec().into();
  bencher.bench_local(move || {
    evm.transact().ok();
  });
  // let result = evm.transact().unwrap();
  // eprintln!("{:#?}", result.result);

  // println!("fib({num}) = {}", rU256::from_be_slice(result.result.output().unwrap()));
}

fn main() {
  divan::main();
}
