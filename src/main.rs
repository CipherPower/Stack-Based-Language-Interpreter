// modules:
mod parser;
mod files;
mod stack;

// includes:
use std::env;
use std::io;
use stack::Stack;
use parser::*;
use files::*;

fn main() {
    // get file from command line args
    let args: Vec<String> = env::args().collect();
    let filename: String = args[1].to_owned();
    drop(args);

    // check if the file has the correct extension
    if !check_extension(&filename, ".sbl") {
        eprintln!("ERROR - Input file must have file type '.sbl'.");
        std::process::exit(-1);
    }

    // read the file into lines:
    let input_file: Result<Vec<String>, io::Error>= read_lines(&filename);
    let input_file = match input_file {
        Ok(lines) => lines,
        Err(_) => {
            eprintln!("ERROR - Could not find input file: '{filename}'.");
            std::process::exit(-1);
        }
    };
    println!("[CMD] File found.");

    // parse the file into a vector of tokens:
    println!("[CMD] Parsing file.");
    let instructions = parse_file(input_file);
    let program = instructions.program;
    let label_tracker = instructions.labels;
    println!("[CMD] Parsing complete.");

    // if the program does not contain at least one exit then the execution will stall, so we
    // check for at least one exit token
    if !program.contains(&Tokens::EXIT) {
        eprintln!("\nERROR - Program does not contain 'EXIT' Keyword.\n");
        std::process::exit(-1);
    }

    // we execute based on the vector of tokens until we reach an exit token.
    println!("[CMD] Execution Beginning.\nPROGRAM:\n");
    let mut pc: usize = 0;
    let mut stack = Stack::new(256);
    while program[pc] != Tokens::EXIT {
        let token = &program[pc];

        match token {
            Tokens::PUSH(num) => {
                stack.push(*num);
                pc += 1;
            },

            Tokens::POP => {
                stack.pop();
                pc += 1;
            },

            Tokens::ADD => {
                let num1 = stack.pop();
                let num2 = stack.pop();
                stack.push(num1 + num2);
                pc += 1;
            },

            Tokens::SUB => {
                let num2 = stack.pop();
                let num1 = stack.pop();
                stack.push(num1 - num2);
                pc += 1;
            }

            Tokens::PRINT(var) => {
                match var {
                    PrintVariants::STR(string_literal) => {
                        println!("{string_literal}");
                        pc += 1;
                    }

                    PrintVariants::TOP => {
                        let num = stack.top();
                        println!("{num}");
                        pc += 1;
                    }

                    PrintVariants::STACK => {
                        let debug_stack = stack.get_stack();
                        println!("{debug_stack:?}");
                        pc += 1;
                    }
                }
            }

            Tokens::READ => {
                let mut input = String::new();
                println!("Input:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("ERROR - Could not read from 'STDIN'.");

                let input = match input.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("ERROR - Invalid input received.\nExpected 'integer' but receive 'string'.");
                        std::process::exit(-1);
                    }
                };

                stack.push(input);
                pc += 1;
            },

            Tokens::DEC => {
                stack.decrement_pointer();
                pc += 1;
            }

            Tokens::INC => {
                stack.increment_pointer();
                pc += 1;
            }

            Tokens::DIV => {
                let num2 = stack.pop();
                let num1 = stack.pop();
                stack.push(num1 / num2);
                pc += 1;
            }

            Tokens::MUL => {
                let num1 = stack.pop();
                let num2 = stack.pop();
                stack.push(num1 * num2);
                pc += 1;
            }

            Tokens::JUMPEQ0(label) => {
                let num = stack.top();
                if num == 0 {
                    pc = label_tracker[label];
                } else {
                    pc += 1;
                }
            }

            Tokens::JUMPGT0(label) => {
                let num = stack.top();
                if num > 0 {
                    pc = label_tracker[label];
                } else {
                    pc += 1;
                }
            }

            Tokens::STACK(size) => {

            }

            Tokens::EXIT => {
                unreachable!();
            }
        }
    }
    println!("\n[CMD] Execution Successful!");
}

