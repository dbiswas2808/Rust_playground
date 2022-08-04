use super::vm_instruction::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub struct AssemblyGenerator {
    stream: BufWriter<File>,
    stack_idx: RegisterFromBase,
}

impl AssemblyGenerator {
    fn generate_memory_load(&mut self, mem_seg: &MemoryType) {
        match mem_seg {
            MemoryType::STACK => {
                self.stream.write(b"@STACK");
            }
            MemoryType::LOCAL => {
                self.stream.write(b"@LOCAL");
            }
            MemoryType::ARGUMENT => {
                self.stream.write(b"@ARGUMENT");
            }
            MemoryType::GLOBAL => {
                self.stream.write(b"@GLOBAL");
            }
            MemoryType::CONSTANT => {
                self.stream.write(b"@CONSTANT");
            }
            MemoryType::STATIC => {
                self.stream.write(b"@STATIC");
            }
            MemoryType::TEMP => {
                self.stream.write(b"@TEMP");
            }
            MemoryType::THIS => {
                self.stream.write(b"@THIS");
            }
            MemoryType::THAT => {
                self.stream.write(b"@THAT");
            }
        }
    }

    pub fn generate_pointer_to_pointee(
        &mut self,
        mem_seg: &MemoryType,
        reg_idx: &RegisterFromBase,
    ) {
        self.generate_memory_load(&mem_seg);
        self.stream.write(format!("A = M + {}", reg_idx.r));
    }

    pub fn generate_push(&mut self, mem_seg: &MemoryType, reg_idx: &RegisterFromBase) {
        self.generate_pointer_to_pointee(&mem_seg, reg_idx);
        self.stream.write(format!("D = M"));
        let stack = MemoryType::STACK;
        self.generate_pointer_to_pointee(&stack, &self.stack_idx);
    }

    pub fn generate_pull(mem_type: MemoryType, reg_idx: RegisterFromBase) {}

    pub fn generate_add(mem_type: MemoryType, reg_idx: RegisterFromBase) {}
}
