use crate::utils::types::{NextAction, ExecutionData};

use super::EVM;

pub mod arithmetic;
pub mod logic;
pub mod memory;
pub mod misc;
pub mod stack;

// 0x00
pub fn stop(_evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    NextAction::Exit(0)
}
