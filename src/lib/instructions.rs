pub type ImmediateValue = i64;
pub type MemoryLocation = usize;
pub type InstructionLocation = usize;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Operand {
    Immediate(ImmediateValue),
    ImmediateAddress(MemoryLocation),
    IntermediateAddress(MemoryLocation),
    Label(InstructionLocation)
}

#[derive(Debug, Clone, PartialEq)]
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