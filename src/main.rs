use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use crate::{
    backends::{
        transpilers::{c::Transpiler as CTranspiler, rust::Transpiler as RustTranspiler},
        vm::{Interpreter, Vm},
    },
    core::{ir::Optimizer, parser::Parser, token::Token, tokenizer::Tokenizer},
};

mod backends;
mod core;

fn main() -> std::io::Result<()> {
    let mut text = String::new();
    let mut file = File::open("./data/mandelbrot.bf")?;
    let _ = file.read_to_string(&mut text)?;

    let tokens = Tokenizer::tokenize(&text);
    println!("{:?}", tokens);

    let expressions = Parser::parse(&tokens);
    println!("{:?}", expressions);

    let expressions = Optimizer::optimize(&expressions);
    println!("{:?}", expressions);

    let code = CTranspiler::transpile(&expressions);
    println!("{}", code);

    let mut file = File::create("./bin/mandelbrot.c")?;
    let _ = file.write_all(code.as_bytes())?;

    Ok(())
}
