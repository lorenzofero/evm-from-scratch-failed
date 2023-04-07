pub mod opcodes;
pub mod utils;

use self::utils::get_opcodes;
use crate::utils::logger::Logger;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

#[derive(Debug)]
pub struct EVM {
    pub stack: Vec<U256>,
    pub pc: usize,
    pub execution_bytecode: Vec<u8>,
}

impl<'a> Logger<'a> for EVM {
    const NAMESPACE: &'a str = "EVM";
}

impl EVM {
    pub fn new() -> EVM {
        let evm = EVM {
            stack: Vec::with_capacity(1024),
            pc: 0,
            execution_bytecode: Vec::new(),
        };

        evm
    }

    pub fn execute(self: &mut Self, bytecode: &str) -> EvmResult {
        let opcodes = get_opcodes();
        self.execution_bytecode = hex::decode(bytecode).unwrap();
        
        EVM::debug(&format!("Execution bytecode: {:x?}", self.execution_bytecode));

        while self.pc < self.execution_bytecode.len() {
            let opcode_num = self
                .execution_bytecode
                .get(self.pc)
                .expect("Could not read bytecode");
            self.pc += 1;

            if *opcode_num == 0 {
                break;
            }

            let opcode = opcodes.get(&opcode_num).expect(&format!(
                "Could not find function associated to opcode_num {:x}",
                opcode_num
            ));

            opcode(self);
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
