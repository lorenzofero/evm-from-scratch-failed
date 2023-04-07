use std::ops::{BitAnd, BitOr, BitXor};

use primitive_types::U256;

use crate::evm::{EVM, utils::{is_negative, flip_sign}};

// 0x10
pub fn lt(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let result = if a < b { U256::from(1) } else { U256::from(0) };

    evm.stack.push(result);
}

// 0x11
pub fn gt(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let result;
    if a > b {
        result = U256::from(1);
    } else {
        result = U256::from(0);
    };

    evm.stack.push(result);
}

// 0x12
pub fn slt(evm: &mut EVM) {
    let mut a = evm.stack.pop().unwrap();
    let mut b = evm.stack.pop().unwrap();

    let is_a_negative = is_negative(&a);
    let is_b_negative = is_negative(&b);

    let result: u8;
    match (is_a_negative, is_b_negative) {
        (true, false) => result = 1,
        (false, true) => result = 0,
        (false, false) => result = if a <= b { 1 } else { 0 },
        (true, true) => {
            flip_sign(&mut a);
            flip_sign(&mut b);
            // now signs are flipped; we check the opposite
            result = if a > b { 1 } else { 0 }
        }
    }

    evm.stack.push(U256::from(result));
}

// 0x13
pub fn sgt(evm: &mut EVM) {
    let mut a = evm.stack.pop().unwrap();
    let mut b = evm.stack.pop().unwrap();

    let is_a_negative = is_negative(&a);
    let is_b_negative = is_negative(&b);

    let result: u8;
    match (is_a_negative, is_b_negative) {
        (true, false) => result = 0,
        (false, true) => result = 1,
        (false, false) => result = if a >= b { 1 } else { 0 },
        (true, true) => {
            flip_sign(&mut a);
            flip_sign(&mut b);
            // now signs are flipped; we check the opposite
            result = if a < b { 1 } else { 0 }
        }
    }

    evm.stack.push(U256::from(result));
}

// 0x14
pub fn eq(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let result = if a == b { 1 } else { 0 };
    evm.stack.push(U256::from(result));
}

// 0x15
pub fn is_zero(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let result = if a.is_zero() { 1 } else { 0 };
    evm.stack.push(U256::from(result));
}

// 0x15
pub fn not(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    evm.stack.push(!a);
}

// 0x16
pub fn and(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.bitand(b))
}

// 0x17
pub fn or(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.bitor(b));
}

pub fn xor(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.bitxor(b));
}
