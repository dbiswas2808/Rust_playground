use std::ops;

// pointer definition for our virtual machine pointer
pub struct VmPointer {
    pub mem_seg: MemorySegment,
    pub register_from_segment_base: i32,
}

impl VmPointer {
    fn load_pointer(self: &Self, stack_pointer: &VmPointer) -> String {
        let mut assembly: String = self.mem_seg.generate_memory_load();
        format!(
            "{}\n{}",
            assembly,
            format!("A = M + {}", self.register_from_segment_base)
        )
    }
}

impl ops::AddAssign<i32> for VmPointer {
    fn add_assign(self: &mut Self, _rhs: i32) {
        self.register_from_segment_base += _rhs;
    }
}

impl ops::SubAssign<i32> for VmPointer {
    fn sub_assign(self: &mut Self, _rhs: i32) {
        self.register_from_segment_base += _rhs;
    }
}

pub struct Push {
    ptr: VmPointer,
}

// impl GenerateAssembly for Push {
//     fn generate_assembly(self: &Self) -> String {
//         self.generate_pointer_ton_pointee(&mem_seg, reg_idx);
//         self.stream.write(format!("D = M"));
//         let stack = MemoryType::STACK;
//         self.generate_pointer_to_pointee(&stack, &self.stack_idx);

//         LoadPointer
//     }
// }

pub struct Pop {
    ptr: VmPointer,
}

pub enum MemoryAccess {
    PUSH(Push),
    POP(Pop),
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
    POINTER(i32),
}

impl MemorySegment {
    pub fn generate_memory_load(self) -> String {
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
            MemorySegment::POINTER(idx) => match idx {
                0 => String::from("@THIS"),
                1 => String::from("@THAT"),
            },
        }
    }
}

pub enum Instruction {
    MemoryAccess(MemoryAccess),
    Arithmetic(Arithmetic),
}
