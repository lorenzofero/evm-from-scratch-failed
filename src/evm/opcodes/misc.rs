use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::{
    evm::{
        utils::{flip_sign, is_negative, update_msize},
        EVM,
    },
    utils::{logger::Logger, types::{NextAction, ExecutionData}},
};

// 0x1b
pub fn shl(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let shift = evm.stack.pop().unwrap();
    let val = evm.stack.pop().unwrap();
    evm.stack.push(val << shift);

    NextAction::Continue
}

// 0x1c
pub fn shr(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let shift = evm.stack.pop().unwrap();
    let val = evm.stack.pop().unwrap();
    evm.stack.push(val >> shift);

    NextAction::Continue
}

// 0x1d
/// This does not work if the number is small and shift is greater or equal 8
pub fn sar(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let shift = evm.stack.pop().unwrap();
    let mut val = evm.stack.pop().unwrap();

    if is_negative(&val) {
        flip_sign(&mut val);
        let mut result = val >> shift;
        flip_sign(&mut result);
        evm.stack.push(result);
    } else {
        evm.stack.push(val >> shift);
    }

    NextAction::Continue
}

// 0x1a
pub fn byte(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let offset = evm.stack.pop().unwrap();
    let val = evm.stack.pop().unwrap();

    if offset >= U256::from(32) {
        EVM::warning("byte offset greater or equal than 32");
        evm.stack.push(U256::zero());
        return NextAction::Continue;
    }

    let byte_offset = 31 - usize::from(offset.byte(0));

    let result = val.byte(byte_offset);
    evm.stack.push(U256::from(result));

    NextAction::Continue
}

// 0x20
pub fn sha3(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let mut hasher = Keccak256::new();

    let starting_offset = evm.stack.pop().unwrap().as_usize();
    let ending_offset = evm.stack.pop().unwrap().as_usize();

    let data = &evm.memory[starting_offset..ending_offset];
    hasher.update(data);

    let hash = hasher.finalize();
    let hash_vec = hash.to_vec();

    let val = U256::from(&hash_vec[..]);
    evm.stack.push(val);

    update_msize(evm, ending_offset);

    NextAction::Continue
}


// 0x5a
/// This is not supported yet, it returns `U256::MAX`
pub fn gas(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    evm.stack.push(U256::MAX);
    NextAction::Continue
}

// 0xfe
pub fn invalid(_evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    NextAction::Exit(1)
}


