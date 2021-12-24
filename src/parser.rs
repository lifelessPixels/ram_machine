use crate::processor::{Instruction, Operand};
use std::{fs::{File}, io::{BufReader, BufRead}};

#[derive(PartialEq)]
enum Entry {
    Label(String),
    Instruction(Instruction),
    UnfinishedInstruction(Instruction, String) // only applicable to jumps (JUMP, JGTZ, JZERO)
}

struct State {
    entries: Vec<Entry>    
}

impl State {
    fn new() -> Self {
        State { 
            entries: Vec::<Entry>::new()  
        }
    }

    fn parse_instruction(&self, line: Vec<String>, original_line: &str, line_number: usize) -> Result<Entry, String> {
        if line.len() > 2 { return Err(format!("too many tokens in \"{}\" at line {}", original_line, line_number)); }

        let argument;
        if line.len() == 2 { 
            let result = parse_argument(line[1].clone(), line_number);
            match result {
                Ok(operand) => argument = Some(operand),
                Err(message) => return Err(message)
            }
        } else { argument = None };

        let instruction_string = line[0].to_lowercase();
        let instruction = instruction_string.as_str();
        if instruction != "halt" && argument.is_none() {
            return Err(format!("no argument provided in \"{}\" at line {}", original_line, line_number));
        }

        match instruction {
            "load" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Load(argument)));
            },
            "store" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                if let Operand::Immediate(_) = argument { return Err(format!("illegal immediate argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Store(argument)));
            },
            "add" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Add(argument)));
            },
            "sub" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Sub(argument)));
            },
            "mult" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Mult(argument)));
            },
            "div" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Div(argument)));
            },
            "read" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                if let Operand::Immediate(_) = argument { return Err(format!("illegal immediate argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Read(argument)));
            },
            "write" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument { return Err(format!("illegal label argument provided in \"{}\" at line {}", original_line, line_number)); }
                return Ok(Entry::Instruction(Instruction::Write(argument)));
            },
            "jump" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument {
                    return Ok(Entry::UnfinishedInstruction(Instruction::Jump(argument), line[1].clone()));
                }
                return Err(format!("illegal non-label argument provided in \"{}\" at line {}", original_line, line_number));
            },
            "jgtz" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument {
                    return Ok(Entry::UnfinishedInstruction(Instruction::Jgtz(argument), line[1].clone()));
                }
                return Err(format!("illegal non-label argument provided in \"{}\" at line {}", original_line, line_number));
            },
            "jzero" => {
                let argument = argument.unwrap();
                if let Operand::Label(_) = argument {
                    return Ok(Entry::UnfinishedInstruction(Instruction::Jzero(argument), line[1].clone()));
                }
                return Err(format!("illegal non-label argument provided in \"{}\" at line {}", original_line, line_number));
            },
            "halt" => {
                Ok(Entry::Instruction(Instruction::Halt))
            },
            _ => return Err(format!("unknown instruction {} at line {}", instruction, line_number))
        }
    }

    fn parse_line(&mut self, line: String, line_number: usize) -> Result<(), String> {
        let original_line = line.clone();
        let line = line.split(";").collect::<Vec<&str>>()[0].trim();
        let line: Vec<String> = line.split_whitespace().map(|s| -> String { s.to_string() }).collect();
        if line.len() == 0 { return Ok(()); } // ignore empty lines
        if line[0].ends_with(":") { // try to parse label
            let label = line[0].trim_end_matches(":");
            if line.len() > 1 { return Err(format!("malformed label definition \"{}\" at line {}", original_line, line_number)); }
            if label.is_empty() { return Err(format!("empty label \"{}\" at line {}", &line[0], line_number)); }
            if self.entries.contains(&Entry::Label(label.to_string())) { return Err(format!("redefined label \"{}\" at line {}", label, line_number)); }
            self.entries.push(Entry::Label(label.to_string()));
        } else { // try to parse instruction
            match self.parse_instruction(line, original_line.as_str(), line_number) {
                Ok(entry) => self.entries.push(entry),
                Err(message) => return Err(message)
            }
        }
        Ok(())
    }

    fn finalize(&self) -> Result<Vec<Instruction>, String> {
        let mut result = Vec::<Instruction>::new();
        for entry in self.entries.iter() {
            match entry {
                Entry::Instruction(instruction) => result.push((*instruction).clone()),
                Entry::UnfinishedInstruction(instruction, label) => {
                    let search_result = self.entries.iter().position(|x| -> bool { *x == Entry::Label(label.clone()) } );
                    match search_result {
                        Some(instruction_pointer) => {
                            match instruction {
                                Instruction::Jump(_) => { result.push(Instruction::Jump(Operand::Label(instruction_pointer))); }
                                Instruction::Jgtz(_) => { result.push(Instruction::Jgtz(Operand::Label(instruction_pointer))); }
                                Instruction::Jzero(_) => { result.push(Instruction::Jzero(Operand::Label(instruction_pointer))); }
                                _ => panic!("unexpected value wrapped in Entry::UnfinishedInstruction")
                            }
                        },
                        None => return Err(format!("label {} not defined", label))
                    }
                }
                Entry::Label(_) => {}
            }
        }
        Ok(result)
    }
}

fn parse_argument(argument: String, line_number: usize) -> Result<Operand, String> {
    if argument.starts_with("=") {
        match argument[1..].parse::<i64>() {
            Ok(value) => return Ok(Operand::Immediate(value)),
            Err(_) => return Err(format!("malformed immediate value {} at line {}", argument, line_number))
        }
    } else if argument.starts_with("^") {
        match argument[1..].parse::<usize>() {
            Ok(value) => return Ok(Operand::IntermediateAddress(value)),
            Err(_) => return Err(format!("malformed intermediate address value {} at line {}", argument, line_number))
        }
    } else {
        match argument.parse::<usize>() {
            Ok(value) => return Ok(Operand::ImmediateAddress(value)),
            Err(_) => return Ok(Operand::Label(0))
        }
    }
}

pub fn parse_input(filepath: &String) -> Result<Vec<Instruction>, String> {
    let file = File::open(filepath);
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let mut state = State::new();
        for (index, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    if let Err(message) = state.parse_line(line, index + 1) {
                        return Err(message);
                    }
                },
                Err(_) => return Err(format!("reading line {} failed", index + 1))
            }
        }
        state.finalize()
    }
    else {
        Err(format!("could not open \"{}\" file to read", filepath))
    }
}
