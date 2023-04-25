use crate::utils::types::NextAction;

use super::EVM;

pub mod arithmetic;
pub mod logic;
pub mod misc;
pub mod stack;
pub mod memory;

// 0x00
pub fn stop(_evm: &mut EVM) -> NextAction {
    NextAction::Exit(0)
}

