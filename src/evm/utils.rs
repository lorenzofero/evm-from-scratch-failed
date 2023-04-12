use crate::{
    evm::{opcodes, EVM},
    utils::{logger::Logger, types::OpcodeFunctions},
};
use primitive_types::U256;
use std::collections::HashMap;

/// Flips the sign of a number using two's complement
pub fn flip_sign(num: &mut U256) {
    *num = !*num + 1;
}

/// Check if the given number is negative according to
/// its binary representation, looking at the MSB
pub fn is_negative(num: &U256) -> bool {
    num.bit(255)
}

fn push_n(evm: &mut EVM, n: u8) {
    let mut str = String::new();
    for _i in 1..=n {
        let byte = evm.execution_bytecode.get(evm.pc).expect("Missing data");
        if byte <= &u8::from(9) {
            str.push_str(&format!("0{}", byte));
        } else {
            str.push_str(&format!("{:x}", byte));
        }
        evm.pc += 1;
    }
    let num = U256::from_str_radix(&str, 16).unwrap();
    evm.stack.push(num);
}

fn generate_push_n_fn(n: u8) -> Box<dyn Fn(&mut EVM)> {
    if n > 32 {
        panic!("ERROR: arg must be a number between 0 and 32 included")
    }

    Box::new(move |evm: &mut EVM| push_n(evm, n))
}

fn dup_n(evm: &mut EVM, n: u8) {
    let mut temp_stack: Vec<U256> = Vec::with_capacity(usize::from(n));

    // pop until we find the value to duplicate
    for _i in 1..n {
        let val = evm.stack.pop().unwrap();
        temp_stack.push(val);
    }

    let val_to_dup = evm.stack.pop().unwrap();
    evm.stack.push(val_to_dup.clone());

    // fill the stack back
    for _i in 1..n {
        let val = temp_stack.pop().unwrap();
        evm.stack.push(val);
    }

    evm.stack.push(val_to_dup);
}

fn generate_dup_n_fn(n: u8) -> Box<dyn Fn(&mut EVM)> {
    if n == 0 || n > 16 {
        EVM::error("Invalid N value for DUP N: it must be between 0 and 16 excluded");
        panic!();
    }

    Box::new(move |evm: &mut EVM| dup_n(evm, n))
}

/// todo this is not good yet
fn swap_n(evm: &mut EVM, n: u8) {
    let mut temp_stack: Vec<U256> = Vec::with_capacity(usize::from(n));
    let first_val = evm.stack.pop().unwrap();

    // pop until we find the value to swap
    for _i in 1..n {
        let val = evm.stack.pop().unwrap();
        temp_stack.push(val);
    }

    let last_val = evm.stack.pop().unwrap();

    evm.stack.push(first_val);

    // fill the stack back
    for _i in 1..n {
        let val = temp_stack.pop().unwrap();
        evm.stack.push(val);
    }

    evm.stack.push(last_val);
}

fn generate_swap_n_fn(n: u8) -> Box<dyn Fn(&mut EVM)> {
    if n == 0 || n > 16 {
        EVM::error("Invalid N value for SWAP N: it must be between 0 and 16 excluded");
        panic!();
    }

    Box::new(move |evm: &mut EVM| swap_n(evm, n))
}

// It'd better to have them static. See PHF crate.
pub fn get_opcodes() -> OpcodeFunctions {
    let mut opcodes: OpcodeFunctions = HashMap::new();

    opcodes.insert(0x00, Box::new(opcodes::stop));
    opcodes.insert(0x01, Box::new(opcodes::arithmetic::add));
    opcodes.insert(0x02, Box::new(opcodes::arithmetic::mul));
    opcodes.insert(0x03, Box::new(opcodes::arithmetic::sub));
    opcodes.insert(0x04, Box::new(opcodes::arithmetic::div));
    opcodes.insert(0x05, Box::new(opcodes::arithmetic::s_div));
    opcodes.insert(0x06, Box::new(opcodes::arithmetic::modulo));
    opcodes.insert(0x07, Box::new(opcodes::arithmetic::s_modulo));
    opcodes.insert(0x08, Box::new(opcodes::arithmetic::add_mod));
    opcodes.insert(0x09, Box::new(opcodes::arithmetic::mul_mod));

    opcodes.insert(0x10, Box::new(opcodes::logic::lt));
    opcodes.insert(0x11, Box::new(opcodes::logic::gt));
    opcodes.insert(0x12, Box::new(opcodes::logic::slt));
    opcodes.insert(0x13, Box::new(opcodes::logic::sgt));
    opcodes.insert(0x14, Box::new(opcodes::logic::eq));
    opcodes.insert(0x15, Box::new(opcodes::logic::is_zero));
    opcodes.insert(0x16, Box::new(opcodes::logic::and));
    opcodes.insert(0x17, Box::new(opcodes::logic::or));
    opcodes.insert(0x18, Box::new(opcodes::logic::xor));
    opcodes.insert(0x19, Box::new(opcodes::logic::not));

    opcodes.insert(0x1b, Box::new(opcodes::misc::shl));
    opcodes.insert(0x1c, Box::new(opcodes::misc::shr));
    opcodes.insert(0x1d, Box::new(opcodes::misc::sar));
    opcodes.insert(0x1a, Box::new(opcodes::misc::byte));

    opcodes.insert(0x0a, Box::new(opcodes::arithmetic::exp));
    // opcodes.insert(0x0b, Box::new(opcodes::sign_extend));
    opcodes.insert(0x50, Box::new(opcodes::pop));

    insert_push_n_functions(&mut opcodes);
    insert_dup_n_functions(&mut opcodes);
    insert_swap_n_functions(&mut opcodes);

    opcodes
}

fn insert_push_n_functions(opcodes: &mut OpcodeFunctions) {
    for n in 1..=32 {
        opcodes.insert(0x5f + n, generate_push_n_fn(n));
    }
}

fn insert_dup_n_functions(opcodes: &mut OpcodeFunctions) {
    for n in 1..=16 {
        opcodes.insert(0x7f + n, generate_dup_n_fn(n));
    }
}

fn insert_swap_n_functions(opcodes: &mut OpcodeFunctions) {
    for n in 1..=16 {
        opcodes.insert(0x8f + n, generate_swap_n_fn(n));
    }
}
