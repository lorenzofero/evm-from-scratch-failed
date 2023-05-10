use primitive_types::U256;

use crate::{
    evm::EVM,
    utils::{
        logger::Logger,
        types::{ExecutionData, NextAction},
    },
};

// 0x40
/// Not yet implemented in tests
pub fn blockhash(_evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    NextAction::Continue
}

// 0x41
pub fn coinbase(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let coinbase = data.block.as_ref().unwrap().coinbase.as_ref().unwrap();

    let val = U256::from_str_radix(coinbase, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x42
pub fn timestamp(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let timestamp = data.block.as_ref().unwrap().timestamp.as_ref().unwrap();

    let val = U256::from_str_radix(timestamp, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x43
pub fn number(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let number = data.block.as_ref().unwrap().number.as_ref().unwrap();

    let val = U256::from_str_radix(number, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x44
pub fn difficulty(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let difficulty = data.block.as_ref().unwrap().difficulty.as_ref().unwrap();

    let val = U256::from_str_radix(difficulty, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x45
pub fn gaslimit(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let gaslimit = data.block.as_ref().unwrap().gaslimit.as_ref().unwrap();

    let val = U256::from_str_radix(gaslimit, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x46
pub fn chain(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    EVM::debug(&format!("stack {:x?}", evm.stack));
    let chain_id = data.block.as_ref().unwrap().chainid.as_ref().unwrap();

    let val = U256::from_str_radix(chain_id, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x47
pub fn selfbalance(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let address = data.tx.as_ref().unwrap().to.as_ref().unwrap();
    let state = data.state.as_ref().unwrap();
    let account_state = state.get(address);

    if let None = account_state {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    }

    let balance = account_state.unwrap().balance.as_ref().unwrap();
    let balance = U256::from_str_radix(balance, 16).unwrap();

    evm.stack.push(balance);

    NextAction::Continue
}

// 0x48
pub fn basefee(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let basefee = data.block.as_ref().unwrap().basefee.as_ref().unwrap();

    let val = U256::from_str_radix(basefee, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}
