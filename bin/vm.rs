use std::{
    fs::File,
    io::{Read, Write},
};

use bf::core::pipeline::Pipeline;
use bf::{
    backends::vm::{Interpreter, Vm},
    core::{parser::Parser, token::Token, tokenizer::Tokenizer},
};

fn main() -> std::io::Result<()> {
    let mut text = String::new();
    let mut file = File::open("./data/programs/copy_02.bf")?;
    let _ = file.read_to_string(&mut text)?;

    let expressions = Pipeline::execute(&text);
    println!("{:?}", expressions);
    let opcodes = Interpreter::interpret(&expressions);
    let mut vm = Vm::from(&opcodes);

    vm.run();

    Ok(())
}
