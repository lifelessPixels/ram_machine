use crate::memory::Memory;

type ImmediateValue = i64;
type MemoryLocation = usize;
type InstructionLocation = usize;

pub trait Tape {
    fn read(&mut self) -> Option<i64>;
    fn write(&mut self, value: i64);
} 

#[derive(Debug)]
#[allow(dead_code)]
pub enum Operand {
    Immediate(ImmediateValue),
    ImmediateAddress(MemoryLocation),
    IntermediateAddress(MemoryLocation),
    Label(InstructionLocation)
}

#[derive(Debug)]
#[allow(dead_code)]
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
    Halt
}

pub struct Processor<T: Tape> {
    instructions: Vec<Instruction>,
    instruction_pointer: InstructionLocation,
    memory: Memory,
    halted: bool,
    tapes: T
}

impl<T: Tape> Processor<T> {
    pub fn new(instructions: Vec<Instruction>, memory_size: usize, tapes: T) -> Self {
        if memory_size == 0 {
            panic!("error: tried to create processor with memory size of 0");
        }
        Processor {
            instructions: instructions,
            instruction_pointer: 0,
            memory: Memory::new(memory_size),
            halted: false,
            tapes: tapes
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn get_intermediate_address(&self, intermediate_address: MemoryLocation) -> Result<MemoryLocation, String> {
        let x = self.memory.get(intermediate_address);
        if x < 0 { return Err("intermediate pointer must be non-negative".to_string()); }
        Ok(x as MemoryLocation)
    }

    // TODO: replace most Err(...) with panic! (after implementing source code parser, if correctly implemented, should not happen)
    pub fn execute_instruction(&mut self) -> Result<(), String> {
        if self.instruction_pointer >= self.instructions.len() {
            self.halted = true;
            return Err("instruction pointer run out of instruction space, processor halted".to_string());
        }
        let current_instruction = &self.instructions[self.instruction_pointer];
        match current_instruction {
            Instruction::Load(operand) => {
                let value_to_load;
                match operand {
                    Operand::Immediate(value) => value_to_load = *value,
                    Operand::ImmediateAddress(value) => value_to_load = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_load = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                        
                    }
                    _ => return Err("load operation cannot be provided with label".to_string())
                }
                self.memory.set(value_to_load, 0);
                self.instruction_pointer += 1;
            },
            Instruction::Store(operand) => {
                let accumulator = self.memory.get(0);
                let address_to_store;
                match operand {
                    Operand::ImmediateAddress(value) => address_to_store = *value,
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => address_to_store = address,
                            Err(message) => return Err(message)
                        };
                    },
                    _ => return Err("store operation cannot be provided with immediate or label".to_string())
                }
                self.memory.set(accumulator, address_to_store);
                self.instruction_pointer += 1;
            },
            Instruction::Add(operand) => {
                let value_to_add;
                match operand {
                    Operand::Immediate(value) => value_to_add = *value,
                    Operand::ImmediateAddress(value) => value_to_add = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_add = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                    }
                    _ => { return Err("add operation cannot be provided with label".to_string()) }
                }
                let new_accumulator = self.memory.get(0) + value_to_add;
                self.memory.set(new_accumulator, 0);
                self.instruction_pointer += 1;
            },
            Instruction::Sub(operand) => {
                let value_to_sub;
                match operand {
                    Operand::Immediate(value) => value_to_sub = *value,
                    Operand::ImmediateAddress(value) => value_to_sub = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_sub = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                    }
                    _ => { return Err("sub operation cannot be provided with label".to_string()) }
                }
                let new_accumulator = self.memory.get(0) - value_to_sub;
                self.memory.set(new_accumulator, 0);
                self.instruction_pointer += 1;
            },
            Instruction::Mult(operand) => {
                let value_to_mult;
                match operand {
                    Operand::Immediate(value) => value_to_mult = *value,
                    Operand::ImmediateAddress(value) => value_to_mult = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_mult = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                    }
                    _ => { return Err("mult operation cannot be provided with label".to_string()) }
                }
                let new_accumulator = self.memory.get(0) * value_to_mult;
                self.memory.set(new_accumulator, 0);
                self.instruction_pointer += 1;
            },
            Instruction::Div(operand) => {
                let value_to_div;
                match operand {
                    Operand::Immediate(value) => value_to_div = *value,
                    Operand::ImmediateAddress(value) => value_to_div = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_div = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                    }
                    _ => { return Err("div operation cannot be provided with label".to_string()) }
                }
                if value_to_div == 0 {
                    return Err("division by zero".to_string());
                }
                let new_accumulator = self.memory.get(0) / value_to_div;
                self.memory.set(new_accumulator, 0);
                self.instruction_pointer += 1;
            },
            Instruction::Read(operand) => {
                let address_to_store;
                match operand {
                    Operand::ImmediateAddress(value) => address_to_store = *value,
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => address_to_store = address,
                            Err(message) => return Err(message)
                        };
                    },
                    _ => return Err("read operation cannot be provided with immediate or label".to_string())
                }
                let tape_value = self.tapes.read();
                match tape_value {
                    Some(value) => self.memory.set(value, address_to_store),
                    None => return Err("tried to read, but end of tape occured".to_string())
                }
                self.instruction_pointer += 1;
            },
            Instruction::Write(operand) => {
                let value_to_write;
                match operand {
                    Operand::Immediate(value) => value_to_write = *value,
                    Operand::ImmediateAddress(value) => value_to_write = self.memory.get(*value),
                    Operand::IntermediateAddress(value) => { 
                        let address = self.get_intermediate_address(*value);
                        match address {
                            Ok(address) => value_to_write = self.memory.get(address),
                            Err(message) => return Err(message)
                        };
                    },
                    _ => return Err("write operation cannot be provided with label".to_string())
                }
                self.tapes.write(value_to_write);
                self.instruction_pointer += 1;
            },
            Instruction::Jump(operand) => {
                match operand {
                    Operand::Label(value) => self.instruction_pointer = *value,
                    _ => return Err("jump operation cannot be provided with immediate, immediate address or intermediate address".to_string())
                }
            },
            Instruction::Jgtz(operand) => {
                match operand {
                    Operand::Label(value) => {
                        if self.memory.get(0) > 0 {
                            self.instruction_pointer = *value
                        } else {
                            self.instruction_pointer += 1;
                        }
                    },
                    _ => return Err("jump operation cannot be provided with immediate, immediate address or intermediate address".to_string())
                }
            },
            Instruction::Jzero(operand) => {
                match operand {
                    Operand::Label(value) => {
                        if self.memory.get(0) == 0 {
                            self.instruction_pointer = *value
                        } else {
                            self.instruction_pointer += 1;
                        }
                    },
                    _ => return Err("jump operation cannot be provided with immediate, immediate address or intermediate address".to_string())
                }
            },
            Instruction::Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("instruction_pointer: {}", self.instruction_pointer);
        self.memory.dump();
    }

}