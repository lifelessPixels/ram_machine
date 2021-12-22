use crate::memory::Memory;

type ImmediateValue = i64;
type MemoryLocation = usize;
type InstructionLocation = usize;

#[derive(Debug)]
pub enum Operand {
    Immediate(ImmediateValue),
    ImmediateAddress(MemoryLocation),
    IntermediateAddress(MemoryLocation),
    Label(InstructionLocation)
}

#[derive(Debug)]
pub enum Instruction {
    Load(Operand),
    Store(Operand),
    Add(Operand),
    Sub(Operand),
    Mult(Operand),
    Div(Operand),
    Read(Operand),
    Write(Operand),
    Jump(Operand),
    Jgtz(Operand),
    Jzero(Operand),
    Halt(Operand)
}

pub struct Processor {
    instructions: Vec<Instruction>,
    instruction_pointer: InstructionLocation,
    memory: Memory,
    halted: bool
}

impl Processor {
    pub fn new(instructions: Vec<Instruction>, memory_size: usize) -> Self {
        if memory_size == 0 {
            panic!("error: tried to create processor with memory size of 0");
        }
        Processor {
            instructions: instructions,
            instruction_pointer: 0,
            memory: Memory::new(memory_size),
            halted: false
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn get_intermediate_address(&self, intermediate_address: MemoryLocation) -> Result<MemoryLocation, &'static str> {
        let x = self.memory.get(intermediate_address);
        if x < 0 { return Err("intermediate pointer must be non-negative"); }
        Ok(x as MemoryLocation)
    }

    pub fn execute_instruction(&mut self) -> Result<(), &str> {
        if self.instruction_pointer >= self.instructions.len() {
            self.halted = true;
            return Ok(());
        }
        let current_instruction = &self.instructions[self.instruction_pointer];
        match current_instruction {
            Instruction::Load(operand) => {
                match operand {
                    Operand::Immediate(value) => { self.memory.set(*value, 0); }
                    Operand::ImmediateAddress(value) => { self.memory.set(self.memory.get(*value), 0); }
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => self.memory.set(self.memory.get(address), 0),
                            Err(message) => return Err(message)
                        };
                        
                    }
                    _ => { return Err("load operation cannot be provided with label") }
                }
                self.instruction_pointer += 1;
            },
            Instruction::Store(operand) => {
                let accumulator = self.memory.get(0);
                match operand {
                    Operand::ImmediateAddress(value) => { self.memory.set(accumulator, *value); }
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => self.memory.set(accumulator, address),
                            Err(message) => return Err(message)
                        };
                    }
                    _ => { return Err("store operation cannot be provided with immediate or label") }
                }
                self.instruction_pointer += 1;
            }
            x => {
                todo!("{:?}: not implemented yet", x);
            }
        }
        Ok(())
    }

    pub fn dump(&self) {
        self.memory.dump();
    }

}