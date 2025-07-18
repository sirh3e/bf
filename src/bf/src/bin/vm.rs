use std::{fs::File, io::Read, time::Instant};

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
    //std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    let mut text = String::new();
    let mut file = File::open(args.file)?;
    let _ = file.read_to_string(&mut text)?;

    let expressions = Pipeline::execute(&text);
    let opcodes = Interpreter::interpret(&expressions);
    let mut vm = Vm::from(&opcodes);

    let start = Instant::now();
    vm.run();
    let duration = start.elapsed();

    println!("{duration:?}");

    Ok(())
}
