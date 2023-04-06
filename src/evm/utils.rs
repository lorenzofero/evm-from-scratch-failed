use crate::{
    evm::{op_functions, EVM},
    utils::types::OpcodeFunctions,
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

/// Reads the first `n` bytes and push it to the stack
pub fn push_n(evm: &mut EVM, n: u8) {
    let mut str = String::new();
    for _i in 1..=n {
        let byte = evm.execution_bytecode.get(evm.pc).expect("Missing data");
        str.push_str(&format!("{:x}", byte));
        evm.pc += 1;
    }
    let num = U256::from_str_radix(&str, 16).unwrap();
    evm.stack.push(num);
}

/// Generates the `n`th push function
pub fn generate_push_n_fn(n: u8) -> Box<dyn Fn(&mut EVM) -> ()> {
    if n > 32 {
        panic!("ERROR: arg must be a number between 0 and 32 included")
    }

    Box::new(move |evm: &mut EVM| push_n(evm, n))
}

pub fn get_opcode_functions() -> OpcodeFunctions {
    let mut opcode_functions: OpcodeFunctions = HashMap::new();

    opcode_functions.insert(0x00, Box::new(op_functions::stop));
    opcode_functions.insert(0x01, Box::new(op_functions::arithmetic::add));
    opcode_functions.insert(0x02, Box::new(op_functions::arithmetic::mul));
    opcode_functions.insert(0x03, Box::new(op_functions::arithmetic::sub));
    opcode_functions.insert(0x04, Box::new(op_functions::arithmetic::div));
    opcode_functions.insert(0x05, Box::new(op_functions::arithmetic::s_div));
    opcode_functions.insert(0x06, Box::new(op_functions::arithmetic::modulo));
    opcode_functions.insert(0x07, Box::new(op_functions::arithmetic::s_modulo));
    opcode_functions.insert(0x08, Box::new(op_functions::arithmetic::add_mod));
    opcode_functions.insert(0x09, Box::new(op_functions::arithmetic::mul_mod));
    opcode_functions.insert(0x10, Box::new(op_functions::arithmetic::lt));
    opcode_functions.insert(0x11, Box::new(op_functions::arithmetic::gt));
    opcode_functions.insert(0x12, Box::new(op_functions::arithmetic::slt));
    opcode_functions.insert(0x13, Box::new(op_functions::arithmetic::sgt));
    opcode_functions.insert(0x14, Box::new(op_functions::arithmetic::eq));
    opcode_functions.insert(0x15, Box::new(op_functions::arithmetic::is_zero));
    opcode_functions.insert(0x0a, Box::new(op_functions::arithmetic::exp));
    // opcode_functions.insert(0x0b, Box::new(op_functions::sign_extend));
    opcode_functions.insert(0x50, Box::new(op_functions::pop));

    insert_push_n_functions(&mut opcode_functions);

    opcode_functions
}

pub fn insert_push_n_functions(opcode_functions: &mut OpcodeFunctions) {
    for n in 1..=32 {
        opcode_functions.insert(0x5f + n, generate_push_n_fn(n));
    }
}
