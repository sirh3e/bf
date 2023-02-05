use std::{
    env::current_dir,
    fs::File,
    io::{Read, Write},
};

use bf::{backends::transpilers::rust::Transpiler, core::pipeline::Pipeline};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program_file: String,

    #[arg(short, long)]
    output_file: String,
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    let mut text = String::new();
    let mut file = File::open(args.program_file)?;
    let _ = file.read_to_string(&mut text)?;

    let expressions = Pipeline::execute(&text);
    let code = Transpiler::transpile(&expressions);

    let mut file = File::create(args.output_file)?;
    let _ = file.write_all(code.as_bytes())?;

    Ok(())
}
