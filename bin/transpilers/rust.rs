use std::{
    fs::File,
    io::{Read, Write},
};

use bf::{
    backends::transpilers::rust::Transpiler,
    core::{ir::Optimizers, parser::Parser, token::Token, tokenizer::Tokenizer},
};

fn main() -> std::io::Result<()> {
    let mut text = String::new();
    let mut file = File::open("./data/programs/mandelbrot.bf")?;
    let _ = file.read_to_string(&mut text)?;

    let tokens = Tokenizer::tokenize(&text);
    let expressions = Parser::parse(&tokens);
    let expressions = Optimizers::optimize(&expressions);

    let code = Transpiler::transpile(&expressions);

    let mut file = File::create("./data/generated/mandelbrot.rs")?;
    let _ = file.write_all(code.as_bytes())?;

    Ok(())
}
