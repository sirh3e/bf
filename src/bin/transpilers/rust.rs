use std::{
    fs::File,
    io::{Read, Write},
};

use bf::{
    backends::transpilers::rust::Transpiler,
    core::{ir::Optimizer, parser::Parser, token::Token, tokenizer::Tokenizer},
};

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

    let code = Transpiler::transpile(&expressions);
    println!("{}", code);

    let mut file = File::create("./bin/mandelbrot.rs")?;
    let _ = file.write_all(code.as_bytes())?;

    Ok(())
}
