use crate::{
    evm::{EVM, utils::update_msize},
    utils::types::{ExecutionData, NextAction},
};

// 0xf3
pub fn return_data(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let offset = evm.stack.pop().unwrap().as_usize();
    let size = evm.stack.pop().unwrap().as_usize();

    let mut str = String::with_capacity(size);

    // to change for load
    for i in 0..=size - 1 {
        let byte = evm.memory[offset + i];
        if byte <= u8::from(15) {
            str.push_str(&format!("0{:x}", byte));
        } else {
            str.push_str(&format!("{:x}", byte));
        }
    }

    evm.result = str;

    update_msize(evm, offset + size - 1);

    NextAction::Continue
}
