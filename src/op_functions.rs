use primitive_types::U256;

use crate::evm::EVM;

pub fn stop(_evm: &mut EVM) {}

pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop().expect("No data");
    let b = evm.stack.pop().expect("No data");
    evm.stack.push(a + b);
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
