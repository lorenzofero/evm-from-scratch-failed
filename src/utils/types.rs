use std::collections::HashMap;

use crate::evm::EVM;

pub type OpcodeFunctions = HashMap<u8, Box<dyn Fn(&mut EVM)>>;
