use super::EVM;

pub mod arithmetic;
pub mod logic;
pub mod misc;

// 0x00
pub fn stop(_evm: &mut EVM) {}

// 0x50
pub fn pop(evm: &mut EVM) {
    evm.stack.pop();
}
