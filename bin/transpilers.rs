use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use bf::{
    backends::transpilers::{
        c::Transpiler as CTranspiler, rust::Transpiler as RustTranspiler, Transpiler,
    },
    core::{ir::Expression, pipeline::Pipeline},
};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    transpiler: EnumTranspiler,

    #[arg(short, long)]
    program_files: Vec<String>,

    #[arg(short, long)]
    output_directory: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum EnumTranspiler {
    C,
    Rust,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let (transpiler, extension): (fn(&[Expression]) -> String, &str) = match args.transpiler {
        EnumTranspiler::C => (CTranspiler::transpile, "c"),
        EnumTranspiler::Rust => (RustTranspiler::transpile, "rs"),
    };

    let _ = args
        .program_files
        .into_iter()
        .map(|program_file| {
            let program_file = Path::new(&program_file);
            let output_program_file_path = {
                let mut path = PathBuf::from(&args.output_directory);
                path.push(program_file.file_name().unwrap());
                path.set_extension(extension);
                path
            };

            read_file_to_string(Path::new(&program_file))
                .map(|code| code_to_expressions(&code))
                .map(|expressions| transpiler(&expressions))
                .map(|code| write_code_to_file(&code, &output_program_file_path))
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    Ok(())
}

#[inline(always)]
fn read_file_to_string(path: &Path) -> std::io::Result<String> {
    let mut text = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut text)?;

    Ok(text)
}

fn code_to_expressions(code: &'_ str) -> Vec<Expression> {
    Pipeline::execute(code)
}

fn write_code_to_file(code: &str, output_file_path: &Path) -> std::io::Result<()> {
    let mut file = File::create(output_file_path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
