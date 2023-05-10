use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::{
    evm::{utils::update_msize, EVM},
    utils::types::{ExecutionData, NextAction},
};

// 0x3b
pub fn extcodesize(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let state = data.state;
    let address = format!("0x{:x}", evm.stack.pop().unwrap());

    if state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let account_state = data.state.as_ref().unwrap().get(&address);

    if account_state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let code = account_state.unwrap().code.as_ref().unwrap().bin.clone();
    let size = U256::from(code.len() / 2);

    evm.stack.push(size);

    NextAction::Continue
}

// 0x3c
pub fn extcodecopy(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let state = data.state;
    let address = format!("0x{:x}", evm.stack.pop().unwrap());
    let dest_offset = evm.stack.pop().unwrap().as_usize();
    let offset = evm.stack.pop().unwrap().as_usize();
    let byte_size = evm.stack.pop().unwrap().as_usize();

    if state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let account_state = data.state.as_ref().unwrap().get(&address);

    if account_state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let code = account_state.unwrap().code.as_ref().unwrap().bin.clone();

    let mut i = 0;
    while i < byte_size {
        let range = std::ops::Range::<usize> {
            start: (offset + i) * 2,
            end: (offset + i) * 2 + 2,
        };

        let byte = if let Some(b) = code.get(range) {
            b
        } else {
            "0"
        };

        let val = u8::from_str_radix(byte, 16).unwrap();
        evm.memory[dest_offset + i] = val;
        i = i + 1;
    }

    update_msize(evm, dest_offset + byte_size - 1);

    NextAction::Continue
}

// 0x3f
pub fn extcodehash(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let address = format!("0x{:x}", evm.stack.pop().unwrap());
    let state = data.state;

    if state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let account_state = data.state.as_ref().unwrap().get(&address);

    if account_state.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    };

    let code = account_state.unwrap().code.as_ref().unwrap().bin.clone();

    let mut hasher = Keccak256::new();
    hasher.update(code);
    let hash = hasher.finalize().to_vec();

    let val = U256::from(&hash[..]);
    evm.stack.push(val);

    NextAction::Continue
}
