use std::collections::HashMap;

use primitive_types::U256;
use serde::Deserialize;

use crate::evm::EVM;

#[derive(Debug)]
pub enum NextAction {
    Continue,
    Exit(u8),
}

pub type Opcode = Box<dyn Fn(&mut EVM, &ExecutionData) -> NextAction>;

pub type Address = String;
pub type Opcodes = HashMap<u8, Opcode>;

pub type State = HashMap<Address, AccountState>;

#[derive(Debug, Deserialize)]
pub struct AccountState {
    pub nonce: Option<String>,
    pub balance: Option<String>,
    pub code: Option<Code>,
}

#[derive(Debug, Deserialize)]
pub struct EvmTest {
    pub name: String,
    pub hint: String,
    pub code: Code,
    pub tx: Option<TxData>,
    pub block: Option<BlockData>,
    pub state: Option<State>,
    pub expect: Expect,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub asm: String,
    pub bin: String,
}

#[derive(Debug, Deserialize)]
pub struct Expect {
    pub stack: Option<Vec<String>>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct TxData {
    pub from: Option<String>,
    pub to: Option<String>,
    pub origin: Option<String>,
    pub gasprice: Option<String>,
    pub value: Option<String>,
    pub data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub basefee: Option<String>,
    pub coinbase: Option<String>,
    pub timestamp: Option<String>,
    pub number: Option<String>,
    pub gaslimit: Option<String>,
    pub difficulty: Option<String>,
    pub chainid: Option<String>,
}

#[derive(Debug)]
pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

#[derive(Debug)]
pub struct ExecutionData<'a> {
    pub bytecode: &'a Vec<u8>,
    pub tx: &'a Option<TxData>,
    pub block: &'a Option<BlockData>,
    pub state: &'a Option<State>,
}
