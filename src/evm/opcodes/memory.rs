use primitive_types::U256;

use crate::{evm::EVM, utils::types::NextAction};

// 0x51
pub fn mload(evm: &mut EVM) -> NextAction {
    let offset = evm.stack.pop().unwrap().as_usize();

    let mut str = String::with_capacity(32);

    // to change for load
    for i in 0..=31 {
        let byte = evm.memory[offset + i];
        if byte <= u8::from(15) {
            str.push_str(&format!("0{:x}", byte));
        } else {
            str.push_str(&format!("{:x}", byte));
        }
    }

    let val = U256::from_str_radix(&str, 16).unwrap();
    evm.stack.push(val);

    NextAction::Continue
}

// 0x52
pub fn mstore(evm: &mut EVM) -> NextAction {
    let offset = evm.stack.pop().unwrap().as_usize();
    let val = evm.stack.pop().unwrap();

    for i in 0..=31 {
        evm.memory[offset + 31 - i] = val.byte(i);
    }

    NextAction::Continue
}

pub fn mstore8(evm: &mut EVM) -> NextAction {
    let offset = evm.stack.pop().unwrap().as_usize();
    let val = evm.stack.pop().unwrap();

    evm.memory[offset] = u8::try_from(val % 256).ok().unwrap();

    NextAction::Continue
}
