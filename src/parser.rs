use std::collections::HashMap;

pub struct Instructions {
    pub program: Vec<Tokens>,
    pub labels: HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
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
    STACK(usize),
    EXIT
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrintVariants {
    TOP,
    STACK,
    STR(String)
}

pub fn parse_file(lines: Vec<String>) -> Instructions {
    let mut program: Vec<Tokens> = Vec::new();
    let mut token_counter: usize = 0;
    let mut label_tracker: HashMap<String, usize> = HashMap::new();

    println!("[CMD] Parsing file.");
    let mut line_number: usize = 0;
    for line in lines.into_iter() {
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

            "STACK" => {
                let size = parts[1].parse::<usize>();
                let size = match size {
                    Ok(size) => size,
                    Err(_) => {
                        eprintln!("ERROR - Unknown symbol line {line_number}.\nExpected 'unsigned int' got 'String'.");
                        std::process::exit(-1);
                    }
                };
                program.push(Tokens::STACK(size));
                token_counter += 1;
            }

            _ => {
                eprintln!("ERROR - Invalid syntax at line: {line_number}.");
                std::process::exit(-1);
            }
        };
    }

    Instructions {
        program,
        labels: label_tracker,
    }

}