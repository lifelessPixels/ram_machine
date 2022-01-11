use std::process::exit;
use std::env::{args, self};
use std::io::{stdin, stdout, BufRead, Write};
use ram_machine::processor::Processor;
use ram_machine::tape::Tape;
use ram_machine::parser::parse_input;

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
        print!("[inp:{}] < ", self.current_input);
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
        println!("[out:{}] > {}", self.current_output, value);
        self.current_output += 1;
    }
}

fn print_usage(program_name: String) {
    println!("usage: ./{} [source_code.rasm] <memory size>", program_name)
}

fn main() {

    let debug_var = env::vars().position(|x| { x.0 == "RAM_DEBUG"});
    let debug_mode = debug_var.is_some();

    let arguments: Vec<String> = args().collect();
    if arguments.len() < 2 {
        print_usage(arguments[0].to_owned());
        exit(1);
    }
    let filename = arguments[1].to_owned();
    let memory_size: usize = if arguments.len() == 3 {
        let memory_size_string = arguments[2].to_owned();
        match memory_size_string.parse::<usize>() {
            Ok(value) => {
                if value == 0 {
                    println!("error: provided memory size cannot be 0");
                    exit(1);
                }
                value
            },
            Err(_) => {
                println!("error: provided memory size of {} is incorrect", memory_size_string);
                exit(1);
            }
        }
    } else { 512 };

    let parse_result = parse_input(&filename);
    if let Err(message) = parse_result {
        println!("parser error: {}", message);
        exit(1);
    }

    let instructions = parse_result.unwrap();
    println!("info: loaded {} instructions, memory size: {} cells", instructions.len(), memory_size);
    let mut processor = Processor::new(
        instructions, 
        memory_size,
        StdTape::new()
    );
    while !processor.is_halted() {
        if debug_mode {
            let state = processor.get_current_state();
            println!("debug: {:?} @ address {}", state.0, state.1);
        }
        if let Err(message) = processor.execute_instruction() {
            println!("execution error: {}", message);
            exit(1);
        }
    }
    // processor.dump();
}

