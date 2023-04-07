use crate::{
    evm::{
        utils::{flip_sign, is_negative},
        EVM,
    },
    utils::logger::Logger,
};

// 0x1b
pub fn shl(evm: &mut EVM) {
    let shift = evm.stack.pop().unwrap();
    let val = evm.stack.pop().unwrap();
    evm.stack.push(val << shift);
}

// 0x1c
pub fn shr(evm: &mut EVM) {
    let shift = evm.stack.pop().unwrap();
    let val = evm.stack.pop().unwrap();
    evm.stack.push(val >> shift);
}

// 0x1d
pub fn sar(evm: &mut EVM) {
    let shift = evm.stack.pop().unwrap();
    let mut val = evm.stack.pop().unwrap();

    if is_negative(&val) {
        flip_sign(&mut val);
        let mut result = val >> shift;
        EVM::debug(&format!("result {}", result));
        flip_sign(&mut result);
        evm.stack.push(result);
    } else {
        evm.stack.push(val >> shift);
    }
}
