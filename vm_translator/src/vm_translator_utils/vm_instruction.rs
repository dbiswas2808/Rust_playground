use std::ops;

pub enum CpuRegisters {
    A,
    M,
    D,
}

impl CpuRegisters {
    pub fn as_string(&self) -> String {
        match self {
            CpuRegisters::A => String::from("A"),
            CpuRegisters::M => String::from("M"),
            CpuRegisters::D => String::from("D"),
        }
    }
}

// Declaration and Definitions for:
// Virtual machine pointer
pub struct VmPointer {
    pub mem_seg: MemorySegment,
    pub register_from_segment_base: i32,
}

impl VmPointer {
    fn get_mem_segment(mem_seg_str: &str) -> Result<MemorySegment, &'static str> {
        match mem_seg_str {
            "local" => Ok(MemorySegment::LOCAL),
            "argument" => Ok(MemorySegment::ARGUMENT),
            "global" => Ok(MemorySegment::GLOBAL),
            "constant" => Ok(MemorySegment::CONSTANT),
            "temp" => Ok(MemorySegment::TEMP),
            "this" => Ok(MemorySegment::THIS),
            "that" => Ok(MemorySegment::THAT),
            "pointer" => Ok(MemorySegment::POINTER),
            "static" => Ok(MemorySegment::STATIC),
            _ => Err("Unknown memory segment string."),
        }
    }

    pub fn from(instruction_str: &[&str]) -> VmPointer {
        VmPointer {
            mem_seg: VmPointer::get_mem_segment(instruction_str[0]).unwrap(),
            register_from_segment_base: instruction_str[1].parse().unwrap(),
        }
    }

    // Generate instructions to load ptr to register M
    pub fn generate_load_pointer(&self) -> String {
        let mem_load: String = self.mem_seg.generate_memory_load();
        let a = CpuRegisters::A.as_string();
        let m = CpuRegisters::M.as_string();
        let r = self.register_from_segment_base;

        format!("{}\n{}", mem_load, format!("{a} = {m} + {r}"))
    }
}

// operator +=
impl ops::AddAssign<i32> for VmPointer {
    #[inline]
    fn add_assign(&mut self, _rhs: i32) {
        self.register_from_segment_base += _rhs;
    }
}

// operator -=
impl ops::SubAssign<i32> for VmPointer {
    fn sub_assign(&mut self, _rhs: i32) {
        self.register_from_segment_base += _rhs;
    }
}

// Implement this trait for generating assembly level instructions
pub trait GenerateAssembly {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String;
}

// Declaration and Definitions for:
// Memory access VM commands
pub struct Push {
    ptr: VmPointer,
}

impl Push {
    pub fn from(instruction_str: &[&str]) -> Push {
        Push {
            ptr: VmPointer::from(&instruction_str[1..]),
        }
    }
}

impl GenerateAssembly for Push {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String {
        let d = CpuRegisters::D.as_string();
        let m = CpuRegisters::M.as_string();
        let load_ptr = self.ptr.generate_load_pointer();
        let copy_to_d = format!("{d} = {m}");
        let stack_load = stack_ptr.generate_load_pointer();
        let copy_to_stack = format!("{m} = {d}");
        *stack_ptr += 1;

        [load_ptr, copy_to_d, stack_load, copy_to_stack].join("\n")
    }
}

pub struct Pop {
    ptr: VmPointer,
}

impl Pop {
    pub fn from(instruction_str: &[&str]) -> Pop {
        Pop {
            ptr: VmPointer::from(&instruction_str[1..]),
        }
    }
}

impl GenerateAssembly for Pop {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String {
        String::from("TODO")
    }
}

pub enum MemoryAccess {
    PUSH(Push),
    POP(Pop),
}

impl MemoryAccess {
    pub fn get_mem_type() {}

    pub fn from(instruction_str: &[&str]) -> Result<MemoryAccess, &'static str> {
        match instruction_str[0] {
            "push" => Ok(MemoryAccess::PUSH(Push::from(&instruction_str))),
            "pop" => Ok(MemoryAccess::POP(Pop::from(&instruction_str))),
            _ => Err("Unknown memory acces instruction"),
        }
    }
}

impl GenerateAssembly for MemoryAccess {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String {
        match self {
            MemoryAccess::PUSH(push) => push.generate_assembly(stack_ptr),
            MemoryAccess::POP(pop) => pop.generate_assembly(stack_ptr),
        }
    }
}

pub enum Arithmetic {
    ADD,
    SUB,
    NEG,
    EQ,
    GET,
    LT,
    AND,
    OR,
    NOT,
}

impl Arithmetic {
    pub fn from(s: &str) -> Result<Arithmetic, &str> {
        match s {
            "add" => Ok(Arithmetic::ADD),
            "sub" => Ok(Arithmetic::SUB),
            "neg" => Ok(Arithmetic::NEG),
            "eq" => Ok(Arithmetic::EQ),
            "get" => Ok(Arithmetic::GET),
            "lt" => Ok(Arithmetic::LT),
            "or" => Ok(Arithmetic::OR),
            "and" => Ok(Arithmetic::AND),
            "not" => Ok(Arithmetic::NOT),
            _ => Err("Command not recognized"),
        }
    }
}

impl GenerateAssembly for Arithmetic {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String {
        String::from("TODO")
    }
}

pub enum MemorySegment {
    STACK,
    STATIC,
    LOCAL,
    ARGUMENT,
    GLOBAL,
    CONSTANT,
    TEMP,
    THIS,
    THAT,
    POINTER,
}

impl MemorySegment {
    pub fn generate_memory_load(&self) -> String {
        match self {
            MemorySegment::STACK => String::from("@STACK"),
            MemorySegment::LOCAL => String::from("@LOCAL"),
            MemorySegment::ARGUMENT => String::from("@ARGUMENT"),
            MemorySegment::GLOBAL => String::from("@GLOBAL"),
            MemorySegment::CONSTANT => String::from("@CONSTANT"),
            MemorySegment::STATIC => String::from("@STATIC"),
            MemorySegment::TEMP => String::from("@TEMP"),
            MemorySegment::THIS => String::from("@THIS"),
            MemorySegment::THAT => String::from("@THAT"),
            MemorySegment::POINTER => String::from("POINTER"),
        }
    }
}

pub enum Instruction {
    MemoryAccess(MemoryAccess),
    Arithmetic(Arithmetic),
}

impl Instruction {
    pub fn from(s: &[&str]) -> Result<Instruction, &'static str> {
        match s.len() {
            1 => Ok(Instruction::Arithmetic(Arithmetic::from(s[0]).unwrap())),
            3 => Ok(Instruction::MemoryAccess(MemoryAccess::from(s).unwrap())),
            _ => Err("Unknown instruction"),
        }
    }
}

impl GenerateAssembly for Instruction {
    fn generate_assembly(self, stack_ptr: &mut VmPointer) -> String {
        match self {
            Instruction::MemoryAccess(inst) => inst.generate_assembly(stack_ptr),
            Instruction::Arithmetic(inst) => inst.generate_assembly(stack_ptr),
        }
    }
}
