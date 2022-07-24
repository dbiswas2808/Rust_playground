use super::vm_instruction::*;
use fancy_regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Parser {
    buf_reader: BufReader<File>,
}

impl Parser {
    pub fn new(file: File) -> Parser {
        let buf_reader = BufReader::new(file);
        Parser { buf_reader }
    }

    fn get_command_enum(s: &str) -> Result<Command, &str> {
        match s {
            "pop" => Ok(Command::POP),
            "push" => Ok(Command::PUSH),
            "add" => Ok(Command::ADD),
            "or" => Ok(Command::OR),
            "and" => Ok(Command::AND),
            "eq" => Ok(Command::EQ),
            _ => Err("Command not recognized"),
        }
    }

    fn get_mem_type(s: &str) -> Result<MemoryType, &str> {
        match s {
            "local" => Ok(MemoryType::LOCAL),
            "static" => Ok(MemoryType::STATIC),
            "global" => Ok(MemoryType::GLOBAL),
            "constant" => Ok(MemoryType::CONSTANT),
            "argument" => Ok(MemoryType::ARGUMENT),
            _ => Err("Unkown memory type"),
        }
    }

    fn get_instruction_if_any(line: &str) -> Option<&str> {
        // upto the begining of comments
        let re = Regex::new(r"^(.*?)(?=/{2}|\n)").unwrap();
        match re.find(&line).expect("Error reading line") {
            Some(val) => Some(val.as_str().trim()),
            None => None,
        }
    }

    fn get_next_instruction_line(self: &mut Self) -> Option<String> {
        loop {
            let mut line = String::new();
            let bytes_read = self
                .buf_reader
                .read_line(&mut line)
                .expect("Error in reading buffer");
            match bytes_read {
                0 => {
                    print!("End of file reached");
                    return None;
                }
                _ => match Self::get_instruction_if_any(&line) {
                    Some(val) => match val {
                        "" => {
                            continue;
                        }
                        _ => {
                            return Some(val.to_string());
                        }
                    },
                    None => {
                        continue;
                    }
                },
            }
        }
    }

    pub fn get_next_instruction(self: &mut Self) -> Result<Option<Instruction>, &str> {
        match self.get_next_instruction_line() {
            Some(instruction_line) => {
                let sp: Vec<&str> = instruction_line.split_whitespace().collect();

                match sp.len() {
                    0 => Ok(None),
                    1 => Ok(Some(Instruction {
                        command: Self::get_command_enum(sp[0]).unwrap(),
                        mem_type: None,
                        register: None,
                        instruction_string: "TODO: line".to_string(),
                    })),
                    3 => {
                        let mem_type: MemoryType = Self::get_mem_type(sp[1]).unwrap();
                        let mem_register = RegisterFromBase {
                            r: sp[2].parse().unwrap(),
                        };
                        return Ok(Some(Instruction {
                            command: Self::get_command_enum(sp[0]).unwrap(),
                            mem_type: Some(mem_type),
                            register: Some(mem_register),
                            instruction_string: "TODO: line".to_string(),
                        }));
                    }
                    _ => return Err("Invalid instruction"),
                }
            }

            None => Ok(None),
        }
    }
}
