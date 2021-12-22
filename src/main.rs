use std::process::exit;

use processor::{Processor, Instruction, Operand};

mod memory;
mod processor;

fn main() {
    let mut processor = Processor::new(vec![
        Instruction::Load(Operand::Immediate(3)),
        Instruction::Store(Operand::ImmediateAddress(1)),
        Instruction::Load(Operand::Immediate(4096)),
        Instruction::Store(Operand::IntermediateAddress(1))
    ], 512);
    while !processor.is_halted() {
        if let Err(message) = processor.execute_instruction() {
            println!("execution error: {}", message);
            exit(1);
        }
    }
    processor.dump();
}
