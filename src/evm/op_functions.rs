use super::EVM;

pub mod arithmetic;

// 0x00
pub fn stop(_evm: &mut EVM) {}

// 0x50
pub fn pop(evm: &mut EVM) {
    evm.stack.pop();
}
