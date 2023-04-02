use std::ops::{BitAnd, BitXor};

use primitive_types::U256;

use crate::evm::EVM;

// 0x00
pub fn stop(_evm: &mut EVM) {}

// 0x01
pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (sum, _flag) = a.overflowing_add(b);
    evm.stack.push(sum);
}

// 0x02
pub fn mul(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (mul, _flag) = a.overflowing_mul(b);
    evm.stack.push(mul);
}

// 0x03
pub fn sub(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (sub, _flag) = a.overflowing_sub(b);
    evm.stack.push(sub);
}

// 0x04
pub fn div(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let zero = U256::zero();
    if b == zero {
        evm.stack.push(zero);
    } else {
        evm.stack.push(a / b);
    }
}

// 0x05
pub fn s_div(evm: &mut EVM) {
    let mut a = evm.stack.pop().unwrap();
    let mut b = evm.stack.pop().unwrap();

    let zero = U256::zero();
    if b == zero {
        return evm.stack.push(zero);
    }

    let is_a_negative = a.bit(255);
    let is_b_negative = b.bit(255);

    // make a and b positive
    if is_a_negative {
        a = !a + 1;
    }
    if is_b_negative {
        b = !b + 1;
    }

    match (is_a_negative, is_b_negative) {
        (false, false) | (true, true) => evm.stack.push(a / b),
        _ => {
            let div = a / b;
            let res_with_sign = !div + 1;
            evm.stack.push(res_with_sign);
        }
    }
}

pub fn modulo(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let zero = U256::zero();
    if b == zero {
        evm.stack.push(zero);
    } else {
        evm.stack.push(a % b);
    }
}

pub fn add_mod(evm: &mut EVM) {
    add(evm);
    modulo(evm);
}

/// May have some problems with very big numbers
/// due to `primitive_types::U256`.
pub fn mul_mod(evm: &mut EVM) {
    mul(evm);
    modulo(evm);
}

pub fn pow(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (res, _flag) = a.overflowing_pow(b);
    evm.stack.push(res);
}

pub fn pop(evm: &mut EVM) {
    evm.stack.pop();
}

pub fn push_1(evm: &mut EVM) {
    push_n(evm, 1);
}

pub fn push_2(evm: &mut EVM) {
    push_n(evm, 2);
}

pub fn push_4(evm: &mut EVM) {
    push_n(evm, 4);
}

pub fn push_6(evm: &mut EVM) {
    push_n(evm, 6);
}

pub fn push_10(evm: &mut EVM) {
    push_n(evm, 10);
}

pub fn push_11(evm: &mut EVM) {
    push_n(evm, 11);
}

pub fn push_32(evm: &mut EVM) {
    push_n(evm, 32);
}

pub fn push_n(evm: &mut EVM, a: u8) {
    let mut str = String::new();
    for _i in 1..=a {
        let byte = evm.execution_bytecode.get(evm.pc).expect("Missing data");
        str.push_str(&format!("{:x}", byte));
        evm.pc += 1;
    }
    let num = U256::from_str_radix(&str, 16).unwrap();
    evm.stack.push(num);
}
