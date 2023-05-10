use primitive_types::U256;

use crate::{
    evm::EVM,
    utils::types::{ExecutionData, NextAction},
};

// 0x54
pub fn sload(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let key = evm.stack.pop().unwrap().as_usize();
    let value = if let Some(v) = evm.storage.get(&key) {
        v.clone()
    } else {
        U256::zero()
    };
    evm.stack.push(value);

    NextAction::Continue
}

// 0x55
pub fn sstore(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let key = evm.stack.pop().unwrap().as_usize();
    let val = evm.stack.pop().unwrap();

    evm.storage.insert(key, val);

    NextAction::Continue
}
