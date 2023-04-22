use std::collections::HashMap;

use crate::evm::EVM;

#[derive(Debug)]
pub enum NextAction {
    Continue,
    Exit(u8),
}

pub type Opcode = Box<dyn Fn(&mut EVM) -> NextAction>;

pub type Opcodes = HashMap<u8, Opcode>;
