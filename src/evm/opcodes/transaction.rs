use primitive_types::U256;

use crate::{
    evm::EVM,
    utils::types::{ExecutionData, NextAction},
};

// 0x30
pub fn address(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let to_address = data.tx.as_ref().unwrap().to.as_ref().unwrap();

    let val = U256::from_str_radix(to_address, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x32
pub fn origin(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let origin = data.tx.as_ref().unwrap().origin.as_ref().unwrap();

    let val = U256::from_str_radix(origin, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x33
pub fn caller(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let from_address = data.tx.as_ref().unwrap().from.as_ref().unwrap();

    let val = U256::from_str_radix(from_address, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x33
pub fn gasprice(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let gasprice = data.tx.as_ref().unwrap().gasprice.as_ref().unwrap();

    let val = U256::from_str_radix(gasprice, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}
