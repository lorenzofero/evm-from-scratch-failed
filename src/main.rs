use evm_from_scratch::{
    evm::EVM,
    utils::types::{EvmTest, ExecutionData},
};
use primitive_types::U256;

fn main() {
    let text = std::fs::read_to_string("./tests.json").unwrap();
    let data: Vec<EvmTest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    let mut evm = EVM::new();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let result = evm.execute(ExecutionData {
            bytecode: &hex::decode(&test.code.bin).unwrap(),
            tx: &test.tx,
            block: &test.block,
        });

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
