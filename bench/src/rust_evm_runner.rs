use std::rc::Rc;

use evm::{
  backend::{RuntimeBackend, RuntimeBaseBackend, RuntimeEnvironment},
  interpreter::{
    error::{CallCreateTrap, ExitError, ExitSucceed},
    etable::Etable,
    machine::Machine,
    runtime::{Context, Log, RuntimeState, TransactionContext},
    EtableInterpreter, RunInterpreter,
  },
};
use primitive_types::{H160, H256, U256};
use revm::primitives::U256 as rU256;

pub struct UnimplementedHandler;

impl RuntimeEnvironment for UnimplementedHandler {
  fn block_hash(&self, _number: U256) -> H256 {
    unimplemented!()
  }
  fn block_number(&self) -> U256 {
    unimplemented!()
  }
  fn block_coinbase(&self) -> H160 {
    unimplemented!()
  }
  fn block_timestamp(&self) -> U256 {
    unimplemented!()
  }
  fn block_difficulty(&self) -> U256 {
    unimplemented!()
  }
  fn block_randomness(&self) -> Option<H256> {
    unimplemented!()
  }
  fn block_gas_limit(&self) -> U256 {
    unimplemented!()
  }
  fn block_base_fee_per_gas(&self) -> U256 {
    unimplemented!()
  }
  fn chain_id(&self) -> U256 {
    unimplemented!()
  }
}

impl RuntimeBaseBackend for UnimplementedHandler {
  fn balance(&self, _address: H160) -> U256 {
    unimplemented!()
  }
  fn code_size(&self, _address: H160) -> U256 {
    unimplemented!()
  }
  fn code_hash(&self, _address: H160) -> H256 {
    unimplemented!()
  }
  fn code(&self, _address: H160) -> Vec<u8> {
    unimplemented!()
  }
  fn storage(&self, _address: H160, _index: H256) -> H256 {
    unimplemented!()
  }
  fn transient_storage(&self, _address: H160, _index: H256) -> H256 {
    unimplemented!()
  }

  fn exists(&self, _address: H160) -> bool {
    unimplemented!()
  }

  fn nonce(&self, _address: H160) -> U256 {
    unimplemented!()
  }
}

impl RuntimeBackend for UnimplementedHandler {
  fn original_storage(&self, _address: H160, _index: H256) -> H256 {
    unimplemented!()
  }

  fn deleted(&self, _address: H160) -> bool {
    unimplemented!()
  }
  fn is_cold(&self, _address: H160, _index: Option<H256>) -> bool {
    unimplemented!()
  }

  fn mark_hot(&mut self, _address: H160, _index: Option<H256>) {
    unimplemented!()
  }

  fn set_storage(&mut self, _address: H160, _index: H256, _value: H256) -> Result<(), ExitError> {
    unimplemented!()
  }
  fn set_transient_storage(
    &mut self,
    _address: H160,
    _index: H256,
    _value: H256,
  ) -> Result<(), ExitError> {
    unimplemented!()
  }
  fn log(&mut self, _log: Log) -> Result<(), ExitError> {
    unimplemented!()
  }
  fn mark_delete(&mut self, _address: H160) {
    unimplemented!()
  }

  fn reset_storage(&mut self, _address: H160) {
    unimplemented!()
  }

  fn set_code(&mut self, _address: H160, _code: Vec<u8>) -> Result<(), ExitError> {
    unimplemented!()
  }

  fn reset_balance(&mut self, _address: H160) {
    unimplemented!()
  }

  fn deposit(&mut self, _address: H160, _value: U256) {
    unimplemented!()
  }
  fn withdrawal(&mut self, _address: H160, _value: U256) -> Result<(), ExitError> {
    unimplemented!()
  }

  fn inc_nonce(&mut self, _address: H160) -> Result<(), ExitError> {
    unimplemented!()
  }
}

static RUNTIME_ETABLE: Etable<RuntimeState, UnimplementedHandler, CallCreateTrap> =
  Etable::runtime();

pub fn run_in_rust_evm(bytecode: &[u8], num: u32) {
  let code: Vec<u8> = bytecode.into();
  let num = rU256::from(num);
  let actual_num = num.saturating_sub(rU256::from(1));
  let data = actual_num.to_be_bytes_vec();

  let mut handler = UnimplementedHandler;

  let machine = Machine::new(
    Rc::new(code),
    Rc::new(data),
    1024,
    10000,
    RuntimeState {
      context: Context {
        address: H160::default(),
        caller: H160::default(),
        apparent_value: U256::default(),
      },
      transaction_context: TransactionContext {
        gas_price: U256::default(),
        origin: H160::default(),
      }
      .into(),
      retbuf: Vec::new(),
    },
  );
  let mut vm = EtableInterpreter::new(machine, &RUNTIME_ETABLE);

  let res = vm.run(&mut handler).exit().unwrap();
  assert_eq!(res, Ok(ExitSucceed::Returned));
}
