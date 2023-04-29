use primitive_types::U256;

use crate::{
    evm::EVM,
    utils::types::{ExecutionData, NextAction},
};

// 0x31
pub fn balance(evm: &mut EVM, data: &ExecutionData) -> NextAction {
    let address = evm.stack.pop().unwrap();
    let address_formatted = &format!("0x{:x}", address);
    let state = data.state.as_ref();

    if let None = state {
        evm.stack.push(U256::zero());
        return NextAction::Continue
    }

    let account_state = state.unwrap().get(address_formatted);

    let balance = match account_state {
        Some(s) => &s.balance,
        None => &None,
    }
    .as_ref();

    let val = match balance {
        Some(b) => U256::from_str_radix(b, 16).unwrap(),
        None => U256::zero(),
    };

    evm.stack.push(val);

    NextAction::Continue
}
