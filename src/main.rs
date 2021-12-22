use std::process::exit;

use processor::{Processor, Instruction, Operand, Tape};

mod memory;
mod processor;

struct TestTape {
    current: i64
}

impl TestTape {
    fn new(starting_value: i64) -> Self {
        TestTape {
            current: starting_value
        }
    }
}

impl Tape for TestTape {
    fn read(&mut self) -> Option<i64> {
        let x = self.current;
        self.current += 1;
        Some(x)
    }
    fn write(&mut self, value: i64) {
        println!("out: {}", value);
    }
}

fn main() {
    let mut processor = Processor::new(
        vec![
            Instruction::Load(Operand::Immediate(3)),
            Instruction::Store(Operand::ImmediateAddress(1)),
            Instruction::Load(Operand::Immediate(4096)),
            Instruction::Store(Operand::IntermediateAddress(1)),
            Instruction::Halt
        ], 
        512,
        TestTape::new(0)
    );
    while !processor.is_halted() {
        if let Err(message) = processor.execute_instruction() {
            println!("execution error: {}", message);
            exit(1);
        }
    }
    processor.dump();
}

