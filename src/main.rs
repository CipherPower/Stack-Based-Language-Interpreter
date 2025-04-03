mod stack;

use std::{collections::HashMap, env, io};
use stack::*;

#[allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
enum PrintVariants {
    TOP,
    STACK,
    STR(String)
}


#[derive(Debug, PartialEq, Eq)]
enum Tokens {
    PUSH(i32),
    POP,
    PRINT(PrintVariants),
    ADD,
    SUB,
    JUMPEQ0(String),
    JUMPGT0(String),
    MUL,
    DIV,
    DEC,
    INC,
    READ,
    EXIT
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args[1].to_owned();
    drop(args);

    if !check_extension(&filename, ".sbl") {
        eprintln!("ERROR - Input file must have file type '.sbl'.");
        std::process::exit(-1);
    }

    let input_file: Result<Vec<String>, std::io::Error>= read_lines(&filename);
    let input_file = match input_file {
        Ok(lines) => lines,
        Err(_) => {
            eprintln!("ERROR - Could not find input file: '{filename}'.");
            std::process::exit(-1);
        }
    };

    println!("[CMD] File found.");

    // time to parse the tokens
    let mut program: Vec<Tokens> = Vec::new();
    let mut token_counter: usize = 0;
    let mut label_tracker: HashMap<String, usize> = HashMap::new();

    println!("[CMD] Parsing file.");
    let mut line_number: usize = 0;
    for line in input_file.into_iter() {
        line_number += 1;
        let parts: Vec<String> = line.split(" ").map(String::from).collect();
        let mut opcode: String = parts[0].to_owned();

        // check for whitespace lines
        if opcode == "" {
            continue
        }

        // checking / defining labels
        if opcode.ends_with(":") {
            opcode = opcode.strip_suffix(":").unwrap().to_string();
            if !label_tracker.contains_key(&opcode) {
                label_tracker.insert(opcode, token_counter);
                continue
            }
            token_counter += 1;
            eprintln!("ERROR LINE {token_counter} - Cannot use label indentifier '{opcode}' twice.");
            std::process::exit(-1);
        } 

        // main program creation
        match opcode.as_str() {
            "PUSH" => {
                let arg: i32 = parts[1].parse::<i32>().unwrap_or(-1);
                program.push(Tokens::PUSH(arg));
                token_counter += 1;
            },

            "POP" => {
                program.push(Tokens::POP);
                token_counter += 1;
            }
            
            "PRINT" => {
                if parts[1] == "TOP" {
                    program.push(Tokens::PRINT(PrintVariants::TOP));
                    token_counter += 1;
                } else if parts[1] == "STACK" {
                    program.push(Tokens::PRINT(PrintVariants::STACK));
                    token_counter += 1;
                } else {
                    let string_lit = parts[1..].join(" ");
                    let string_lit = string_lit[1..string_lit.len()-1].to_owned();
                    program.push(Tokens::PRINT(PrintVariants::STR(string_lit)));
                    token_counter += 1;
                }
            }

            "ADD" => {
                program.push(Tokens::ADD);
                token_counter += 1;
            }

            "SUB" => {
                program.push(Tokens::SUB);
                token_counter += 1;
            }

            "JUMP_EQ_0" => {
                let label = parts[1].to_owned();
                program.push(Tokens::JUMPEQ0(label));
                token_counter += 1;
            }

            "JUMP_GT_0" => {
                let label = parts[1].to_owned();
                program.push(Tokens::JUMPGT0(label));
                token_counter += 1;
            }

            "MUL" => {
                program.push(Tokens::MUL);
                token_counter += 1;
            }

            "DIV" => {
                program.push(Tokens::DIV);
                token_counter += 1;
            }

            "INC" => {
                program.push(Tokens::INC);
                token_counter += 1;
            }

            "DEC" => {
                program.push(Tokens::DEC);
                token_counter += 1;
            }

            "READ" => {
                program.push(Tokens::READ);
                token_counter += 1;
            }
            
            "EXIT" => {
                program.push(Tokens::EXIT);
                token_counter += 1;
            }

            _ => {
                eprintln!("ERROR - Invalid syntax at line: {line_number}.");
                std::process::exit(-1);
            }
        };
    }

    println!("[CMD] Parsing complete.");

    // Basic error checking:
    if !program.contains(&Tokens::EXIT) {
        eprintln!("\nERROR - Program does not contain 'EXIT' Keyword.\n");
        std::process::exit(-1);
    }

    let mut pc: usize = 0;
    let mut stack = Stack::new(256);

    println!("[CMD] Execution Beginning.\nPROGRAM:\n");

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

            Tokens::EXIT => {
                unreachable!();
            }
        }
    }
    println!("\n[CMD] Execution Succesful!");
}

fn check_extension(str: &str, extension: &str) -> bool {
    let file_ext: usize = str.len() - 4;
    if str[file_ext..] != *extension {
        false
    } else {
        true
    }
 }

 fn read_lines(file_name: &str) -> std::io::Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(file_name)?
            .lines()
            .map(String::from)
            .collect()
    )
}