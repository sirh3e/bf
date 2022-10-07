use std::fs;
use std::io::Read;

use crate::backends::{
    transpilers::rust::Transpiler,
    vm::{Interpreter, Vm},
};
use crate::ir::Optimizer;
use crate::parser::Parser;
use crate::token::Token;
use crate::tokenizer::Tokenizer;

mod backends;
mod ir;
mod parser;
mod token;
mod tokenizer;

fn main() -> std::io::Result<()> {
    let mut text = String::new();
    let mut file =
        fs::File::open("/home/sirh3e/Programming/vcs/git/local/rust/bf/bin/mandelbrot.bf")?;
    let _ = file.read_to_string(&mut text)?;

    let tokens = Tokenizer::tokenize(&text);
    println!("{:?}", tokens);

    let expressions = Parser::parse(&tokens);
    println!("{:?}", expressions);

    let expressions = Optimizer::optimize(&expressions);
    println!("{:?}", expressions);

    let opcodes = Interpreter::interpret(&expressions);
    println!("{:?}", opcodes);

    let mut vm = Vm::from(&opcodes);
    vm.run();

    Ok(())
}
