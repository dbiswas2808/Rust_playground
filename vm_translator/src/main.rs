use std::fs::File;
use vm_translator_utils::Parser;
mod vm_translator_utils;

fn main() {
    let fname = "/Users/deepanjanbiswas/Documents/GitHub/RustDeepDive/HttpServer/vm_translator/sample_test.vm";
    let file = File::open(fname).expect("Error opening file");
    let mut parse = Parser::new(file);
    let line1 = parse.get_next_instruction();
    let line2 = parse.get_next_instruction();
    let line3 = parse.get_next_instruction();
    let line4 = parse.get_next_instruction();
}
