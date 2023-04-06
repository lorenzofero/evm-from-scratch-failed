pub mod op_functions;
pub mod utils;

use crate::utils::logger::Logger;
use primitive_types::U256;
use self::utils::get_opcode_functions;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

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

    /// Executes the given hex bytecode string, and cleans the stack and
    /// pc before returning `EvmResult`.
    pub fn execute(self: &mut Self, bytecode: &str) -> EvmResult {
        // TODO: in this way, for each execution opcodes are created.
        let opcode_functions = get_opcode_functions();
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

            let op_function = opcode_functions.get(&opcode).expect(&format!(
                "Could not find function associated to opcode {:x}",
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
