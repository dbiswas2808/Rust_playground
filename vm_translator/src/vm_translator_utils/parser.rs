use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

pub struct Parser {
    buf_reader: BufReader<File>,
}

pub enum Command {
    POP,
    PUSH,
    ADD,
    SUB,
    EQ,
    OR,
    AND,
}

pub enum MemoryType {
    STATIC,
    LOCAL,
    ARGUMENT,
    GLOBAL,
    CONSTANT,
    TEMP,
    THIS,
    THAT,
}

struct MemoryRegister {
    r: i64,
}

pub struct Instruction {
    command: Command,
    mem_type: Option<MemoryType>,
    register: Option<MemoryRegister>,
    instruction_string: String,
}

impl Parser {
    pub fn new(_fname: &str) -> Parser {
        let file = File::open(_fname).expect("Error opening file");
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

    fn get_instructon_from_line(line: &str)->&str {
        // remove comments and new line character
        let re = Regex::new(r"^(.*?)(?=/{2}|\n)").unwrap();
        let caps = re.captures(&line).unwrap();
        caps.get(1).map_or("", |m| m.as_str())
    }

    pub fn get_next_instruction(self: &mut Self) -> Result<Option<Instruction>, &str> {
        let mut line = String::new();
        let len = self
            .buf_reader
            .read_line(&mut line)
            .expect("Error in reading buffer");
        if len == 0 {
            print!("End of file reached");
            return Ok(None);
        }

        let instruction_line = Self::get_instructon_from_line(&line);
        if instruction_line.len() == 0 {
            // No instruction
            return Ok(None);
        }

        let sp: Vec<&str> = instruction_line.split(" ").collect();
        let command = Self::get_command_enum(sp[0]).unwrap();
        if sp.len() == 3 {
            let mem_type: MemoryType = Self::get_mem_type(sp[1]).unwrap();
            let mem_register = MemoryRegister {
                r: sp[2].parse().unwrap(),
            };
            return Ok(Some(Instruction {
                command: command,
                mem_type: Some(mem_type),
                register: Some(mem_register),
                instruction_string: line,
            }));
        } else if sp.len() > 3 {
            return Err("Not a valid instruction");
        }

        Ok(Some(Instruction {
            command: command,
            mem_type: None,
            register: None,
            instruction_string: line,
        }))
    }
}
