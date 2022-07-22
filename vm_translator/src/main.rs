use vm_translator_utils::Parser;

mod vm_translator_utils;

fn main() {
    let mut parse = Parser::new("/Users/deepanjanbiswas/Documents/GitHub/RustDeepDive/HttpServer/vm_translator/sample_test.vm");
    let line1 = parse.get_next_instruction();
    let line2 = parse.get_next_instruction();
    let line3 = parse.get_next_instruction();
}
