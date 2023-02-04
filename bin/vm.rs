use std::{
    fs::File,
    io::{Read, Write},
};

use bf::{
    backends::vm::{Interpreter, Vm},
    core::pipeline::Pipeline,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut text = String::new();
    let mut file = File::open(args.file)?;
    let _ = file.read_to_string(&mut text)?;

    let expressions = Pipeline::execute(&text);
    let opcodes = Interpreter::interpret(&expressions);
    let mut vm = Vm::from(&opcodes);

    vm.run();

    Ok(())
}
