use crate::evm::utils::flip_sign;
use crate::evm::utils::is_negative;
use primitive_types::U256;

use crate::evm::EVM;

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

    let is_a_negative = is_negative(&a);
    let is_b_negative = is_negative(&b);

    // make a and b positive
    if is_a_negative {
        flip_sign(&mut a);
    }
    if is_b_negative {
        flip_sign(&mut b);
    }

    match (is_a_negative, is_b_negative) {
        (false, false) | (true, true) => evm.stack.push(a / b),
        _ => {
            let mut div = a / b;
            flip_sign(&mut div);
            evm.stack.push(div);
        }
    }
}

// 0x06
pub fn modulo(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();
    let zero = U256::zero();
    if n == zero {
        evm.stack.push(zero);
    } else {
        evm.stack.push(a % n);
    }
}

// 0x07
pub fn s_modulo(evm: &mut EVM) {
    let mut a = evm.stack.pop().unwrap();
    let mut n = evm.stack.pop().unwrap();

    let zero = U256::zero();
    if n == zero {
        return evm.stack.push(zero);
    }

    let is_a_negative = is_negative(&a);
    let is_n_negative = is_negative(&n);

    // Recall that $$ka \equiv kb (\mod n)$$ for any integer $k$
    if is_a_negative {
        flip_sign(&mut a);
    }
    if is_n_negative {
        flip_sign(&mut n);
    }

    let mut result = a % n;
    match (is_a_negative, is_n_negative) {
        (false, false) => {
            evm.stack.push(result);
        }
        // Consider an example where a = 10, n = -3 and flip such signs
        _ => {
            flip_sign(&mut result);
            evm.stack.push(result);
        }
    }
}

// 0x08
pub fn add_mod(evm: &mut EVM) {
    add(evm);
    modulo(evm);
}

// 0x09
/// May have some problems with very big numbers
/// due to `primitive_types::U256`.
pub fn mul_mod(evm: &mut EVM) {
    mul(evm);
    modulo(evm);
}

// 0xa
pub fn exp(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (res, _flag) = a.overflowing_pow(b);
    evm.stack.push(res);
}
