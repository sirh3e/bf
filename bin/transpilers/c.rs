use std::{
    env::current_dir,
    fs::File,
    io::{Read, Write},
};

use bf::{
    backends::transpilers::c::Transpiler,
    core::{ir::Optimizers, parser::Parser, token::Token, tokenizer::Tokenizer},
};

fn main() -> std::io::Result<()> {
    let current_path_buffer = current_dir()?;
    let current_path = current_path_buffer.as_path().as_os_str();
    println!("{:?}", current_path);

    let mut text = String::new();
    let mut file = File::open("./data/programs/mandelbrot.bf")?;
    let _ = file.read_to_string(&mut text)?;

    let tokens = Tokenizer::tokenize(&text);
    let expressions = Parser::parse(&tokens);
    let expressions = Optimizers::optimize(&expressions);

    let code = Transpiler::transpile(&expressions);

    let mut file = File::create("./data/generated/mandelbrot.c")?;
    let _ = file.write_all(code.as_bytes())?;

    Ok(())
}
