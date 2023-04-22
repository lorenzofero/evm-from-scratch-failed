use primitive_types::U256;

use crate::{evm::EVM, utils::types::NextAction};

// 0x50
pub fn pop(evm: &mut EVM) -> NextAction {
    evm.stack.pop();
    NextAction::Continue
}

// 0x56
pub fn jump(evm: &mut EVM) -> NextAction {
    let new_pc = evm.stack.pop().unwrap().as_usize();

    evm.pc = new_pc;

    NextAction::Continue
}

// 0x58
pub fn pc(evm: &mut EVM) -> NextAction {
    evm.stack.push(U256::from(evm.pc - 1));
    NextAction::Continue
}

// 0x5B
pub fn jumpdest(_evm: &mut EVM) -> NextAction {
    NextAction::Continue
}
