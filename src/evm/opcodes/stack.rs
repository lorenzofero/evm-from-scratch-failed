use primitive_types::U256;

use crate::{
    evm::{utils::is_pc_on_jumpdest, EVM},
    utils::types::{NextAction, ExecutionData},
};

// 0x50
pub fn pop(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    evm.stack.pop();
    NextAction::Continue
}

// 0x56
pub fn jump(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let new_pc = evm.stack.pop().unwrap().as_usize();

    evm.pc = new_pc;

    if is_pc_on_jumpdest(evm) {
        NextAction::Continue
    } else {
        // TODO: maybe another status code for this?
        NextAction::Exit(1)
    }
}

// 0x57
pub fn jumpi(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let new_pc = evm.stack.pop().unwrap().as_usize();
    let condition = evm.stack.pop().unwrap();

    if !condition.is_zero() {
        evm.pc = new_pc;
    } else {
        return NextAction::Continue;
    }

    if is_pc_on_jumpdest(evm) {
        NextAction::Continue
    } else {
        // TODO: maybe another status code for this?
        NextAction::Exit(1)
    }
}

// 0x58
pub fn pc(evm: &mut EVM, __data: &ExecutionData) -> NextAction {
    evm.stack.push(U256::from(evm.pc - 1));
    NextAction::Continue
}

// 0x5B
pub fn jumpdest(_evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    NextAction::Continue
}
