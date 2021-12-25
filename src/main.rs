mod memory;
mod processor;
mod parser;

use std::process::exit;
use std::env::{args};
use std::io::{stdin, stdout, BufRead, Write};
use processor::{Processor, Tape};
use parser::parse_input;

struct StdTape {
    current_input: usize,
    current_output: usize
}

impl StdTape {
    fn new() -> Self {
        Self {
            current_input: 0,
            current_output: 0
        }
    }
}

impl Tape for StdTape {
    fn read(&mut self) -> Option<i64> {
        print!("[{}] < ", self.current_input);
        self.current_input += 1;
        if let Err(_) = stdout().flush() { }
        match stdin().lock().lines().next() {
            Some(result) => {
                match result {
                    Ok(read_line) => {
                        match read_line.trim().parse::<i64>() {
                            Ok(value) => { return Some(value); },
                            Err(_) => { return None }
                        }
                    },
                    Err(_) => { return None }
                }
            },
            None => { return None }
        }
    }
    fn write(&mut self, value: i64) {
        println!("[{}] > {}", self.current_output, value);
        self.current_output += 1;
    }
}

fn print_usage(program_name: String) {
    println!("usage: ./{} <source_code.rasm>", program_name)
}

fn main() {

    let arguments: Vec<String> = args().collect();
    if arguments.len() != 2 {
        print_usage(arguments[0].to_owned());
        exit(1);
    }
    let filename = arguments[1].to_owned();

    let parse_result = parse_input(&filename);
    if let Err(message) = parse_result {
        println!("parser error: {}", message);
        exit(1);
    }

    let instructions = parse_result.unwrap();
    let mut processor = Processor::new(
        instructions, 
        512,
        StdTape::new()
    );
    while !processor.is_halted() {
        if let Err(message) = processor.execute_instruction() {
            println!("execution error: {}", message);
            exit(1);
        }
    }
    // processor.dump();
}

