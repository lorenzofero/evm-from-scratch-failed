use crate::utils::types::NextAction;

use super::EVM;

pub mod arithmetic;
pub mod logic;
pub mod misc;

// 0x00
pub fn stop(_evm: &mut EVM) -> NextAction {
    NextAction::Exit(0)
}

// 0x50
pub fn pop(evm: &mut EVM) -> NextAction {
    evm.stack.pop();
    NextAction::Continue
}
