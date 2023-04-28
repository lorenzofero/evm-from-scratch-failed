use crate::evm::utils::flip_sign;
use crate::evm::utils::is_negative;
use crate::utils::types::ExecutionData;
use crate::utils::types::NextAction;
use primitive_types::U256;

use crate::evm::EVM;

// 0x01
pub fn add(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (sum, _flag) = a.overflowing_add(b);
    evm.stack.push(sum);

    NextAction::Continue
}

// 0x02
pub fn mul(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (mul, _flag) = a.overflowing_mul(b);
    evm.stack.push(mul);

    NextAction::Continue
}

// 0x03
pub fn sub(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (sub, _flag) = a.overflowing_sub(b);
    evm.stack.push(sub);

    NextAction::Continue
}

// 0x04
pub fn div(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let zero = U256::zero();
    if b == zero {
        evm.stack.push(zero);
    } else {
        evm.stack.push(a / b);
    }

    NextAction::Continue
}

// 0x05
pub fn s_div(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let mut a = evm.stack.pop().unwrap();
    let mut b = evm.stack.pop().unwrap();

    let zero = U256::zero();
    if b == zero {
        evm.stack.push(zero);
        return NextAction::Continue;
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

    NextAction::Continue
}

// 0x06
pub fn modulo(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();
    let zero = U256::zero();
    if n == zero {
        evm.stack.push(zero);
    } else {
        evm.stack.push(a % n);
    }

    NextAction::Continue
}

// 0x07
pub fn s_modulo(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let mut a = evm.stack.pop().unwrap();
    let mut n = evm.stack.pop().unwrap();

    let zero = U256::zero();
    if n == zero {
        evm.stack.push(zero);
        return NextAction::Continue;
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

    NextAction::Continue
}

// 0x08
pub fn add_mod(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    add(evm, _data);
    modulo(evm, _data);

    NextAction::Continue
}

// 0x09
/// May have some problems with very big numbers
/// due to `primitive_types::U256`.
pub fn mul_mod(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    mul(evm, _data);
    modulo(evm, _data);

    NextAction::Continue
}

// 0xa
pub fn exp(evm: &mut EVM, _data: &ExecutionData) -> NextAction {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let (res, _flag) = a.overflowing_pow(b);
    evm.stack.push(res);

    NextAction::Continue
}
