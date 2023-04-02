use primitive_types::U256;

use crate::evm::EVM;

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

    let foo = Box::new(move |evm: &mut EVM| push_n(evm, n));
    foo
}
