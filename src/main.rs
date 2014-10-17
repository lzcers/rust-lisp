#![feature(if_let)]
#![feature(globs)]

use std::io::stdio;
use reader::SexpReader;
use interpreter::Interpreter;

mod reader;
mod interpreter;
mod intrinsics;

#[allow(dead_code)]
fn main() {
    let mut stdin = stdio::stdin();
    let mut interpreter = Interpreter::new();
    loop {
        print!("scheme> ");
        if let Ok(line) = stdin.read_line() {
            process_line(line, &mut interpreter);
        } else {
            println!("\nBye!");
            break;
        }
    }
}


fn process_line(input: String, interpreter: &mut Interpreter) {
    let mut reader = SexpReader::new();
    match reader.parse_str(input.as_slice()) {
        Ok(e) => match interpreter.eval(&e)  {
            Ok(val) => println!("==> {}", val),
            Err(e) => println!("error: {}", e)
        },
        Err(e) => println!("error: {}", e)
    };
}







