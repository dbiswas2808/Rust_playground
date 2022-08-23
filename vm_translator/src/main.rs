use std::fs::File;
use std::io::Write;
use vm_translator_utils::vm_instruction::*;
use vm_translator_utils::Parser;
mod vm_translator_utils;

fn main() {
    let fname = "/Users/deepanjanbiswas/Documents/GitHub/RustDeepDive/HttpServer/vm_translator/sample_test.vm";
    let file = File::open(fname).expect("Error opening file");
    let mut parse = Parser::new(file);

    let mut stack_ptr = VmPointer {
        mem_seg: MemorySegment::STACK,
        register_from_segment_base: 0,
    };

    // let mut s: String;
    // match parse.get_next_instruction() {
    //     Some(line) => {
    //         s = line.generate_assembly(stack_ptr);
    //     }
    //     None => (),
    // }

    let line2 = parse.get_next_instruction();
    let assembly = line2.unwrap().generate_assembly(&mut stack_ptr);

    let fname_assembly = "/Users/deepanjanbiswas/Documents/GitHub/RustDeepDive/HttpServer/vm_translator/sample_test_assembly.ass";
    let mut file = File::create(fname_assembly).expect("Error opening file");
    file.write(assembly.as_bytes());
    // let line3 = parse.get_next_instruction();
    // let line4 = parse.get_next_instruction();
}
