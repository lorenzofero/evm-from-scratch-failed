use primitive_types::U256;
use std::collections::HashMap;

use crate::op_functions;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub struct EVM {
    pub stack: Vec<U256>,
    pub pc: usize,
    pub execution_bytecode: Vec<u8>,
    opcodes_functions: HashMap<u8, fn(&mut self::EVM) -> ()>,
}

impl EVM {
    pub fn new() -> EVM {
        let mut evm = EVM {
            stack: Vec::with_capacity(1024),
            pc: 0,
            opcodes_functions: HashMap::new(),
            execution_bytecode: Vec::new(),
        };

        evm.opcodes_functions.insert(0x00, op_functions::stop);
        evm.opcodes_functions.insert(0x01, op_functions::add);
        evm.opcodes_functions.insert(0x02, op_functions::mul);
        evm.opcodes_functions.insert(0x03, op_functions::sub);
        evm.opcodes_functions.insert(0x04, op_functions::div);
        evm.opcodes_functions.insert(0x05, op_functions::s_div);
        evm.opcodes_functions.insert(0x06, op_functions::modulo);
        evm.opcodes_functions.insert(0x07, op_functions::s_modulo);
        evm.opcodes_functions.insert(0x08, op_functions::add_mod);
        evm.opcodes_functions.insert(0x09, op_functions::mul_mod);
        evm.opcodes_functions.insert(0x10, op_functions::lt);
        evm.opcodes_functions.insert(0x11, op_functions::gt);
        evm.opcodes_functions.insert(0x12, op_functions::slt);
        evm.opcodes_functions.insert(0x0a, op_functions::exp);
        // evm.opcodes_functions.insert(0x0b, op_functions::sign_extend);
        evm.opcodes_functions.insert(0x50, op_functions::pop);
        evm.opcodes_functions.insert(0x60, op_functions::push_1);
        evm.opcodes_functions.insert(0x61, op_functions::push_2);
        evm.opcodes_functions.insert(0x63, op_functions::push_4);
        evm.opcodes_functions.insert(0x65, op_functions::push_6);
        evm.opcodes_functions.insert(0x69, op_functions::push_10);
        evm.opcodes_functions.insert(0x6a, op_functions::push_11);
        evm.opcodes_functions.insert(0x7f, op_functions::push_32);

        evm
    }

    /// Executes the given hex bytecode string, and cleans the stack and
    /// pc before returning `EvmResult`.
    pub fn execute(self: &mut Self, bytecode: &str) -> EvmResult {
        self.execution_bytecode = hex::decode(bytecode).unwrap();

        while self.pc < self.execution_bytecode.len() {
            let opcode = self
                .execution_bytecode
                .get(self.pc)
                .expect("Could not read bytecode");
            self.pc += 1;

            if *opcode == 0 {
                break;
            }

            let op_function = self.opcodes_functions.get(&opcode).expect(&format!(
                "Could not find function associated to opcode {}",
                opcode
            ));

            op_function(self);
        }

        let result = self.get_result();
        self.reset();

        result
    }

    fn reset(&mut self) -> () {
        self.pc = 0;
        self.stack.clear();
    }

    fn get_result(&self) -> EvmResult {
        let mut clone = self.stack.clone();
        clone.reverse();

        EvmResult {
            stack: clone,
            success: true,
        }
    }
}
