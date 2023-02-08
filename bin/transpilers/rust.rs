use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use bf::{backends::transpilers::rust::Transpiler, core::pipeline::Pipeline};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program_file: String,

    #[arg(short, long)]
    output_directory: String,
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    let mut text = String::new();
    let path = Path::new(&args.program_file);
    let mut file = File::open(&args.program_file)?;
    let _ = file.read_to_string(&mut text)?;

    let expressions = Pipeline::execute(&text);
    let code = Transpiler::transpile(&expressions);

    let mut path_buf = PathBuf::from(&args.output_directory);
    path_buf.push(path.file_name().unwrap());
    path_buf.set_extension("rs");

    let output_file_path = path_buf.as_path();
    let mut file = File::create(output_file_path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
