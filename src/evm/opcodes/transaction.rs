use primitive_types::U256;

use crate::{
    evm::{utils::update_msize, EVM},
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

// 0x34
pub fn callvalue(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let value = data.tx.as_ref().unwrap().value.as_ref().unwrap();

    let val = U256::from_str_radix(value, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x35
pub fn calldataload(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let offset = evm.stack.pop().unwrap().as_usize();
    let data = data.tx.as_ref().unwrap().data.as_ref().unwrap();
    let mut parsed_data = String::with_capacity(32);

    for i in 0..=63 {
        let j = i + offset * 2;
        if let Some(v) = data.get(j..j + 1) {
            parsed_data.push_str(v)
        } else {
            parsed_data.push_str("0")
        };
    }

    let val = U256::from_str_radix(&parsed_data, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x36
pub fn calldatasize(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    if data.tx.is_none() {
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    }

    let data = data.tx.as_ref().unwrap().data.as_ref().unwrap();

    // data.len is even, every two chars is a byte
    let size = data.len() / 2;

    let val = U256::from(size);
    evm.stack.push(val);

    NextAction::Continue
}

// 0x37
pub fn calldatacopy(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let dest_offset = evm.stack.pop().unwrap().as_usize();
    let offset = evm.stack.pop().unwrap().as_usize();
    let byte_size = evm.stack.pop().unwrap().as_usize();

    let str_data = data.tx.as_ref().unwrap().data.as_ref().unwrap();

    let mut i = 0;
    while i < byte_size {
        let range = std::ops::Range::<usize> {
            start: (offset + i) * 2,
            end: (offset + i) * 2 + 2,
        };
        let byte = str_data.get(range).unwrap();
        let val = u8::from_str_radix(byte, 16).unwrap();
        evm.memory[dest_offset + i] = val;
        i = i + 1;
    }

    update_msize(evm, dest_offset + byte_size - 1);

    NextAction::Continue
}

// 0x3a
pub fn gasprice(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let gasprice = data.tx.as_ref().unwrap().gasprice.as_ref().unwrap();

    let val = U256::from_str_radix(gasprice, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}
