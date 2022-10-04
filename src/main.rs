use std::fs;

use crate::backends::transpilers::rust::Transpiler;
use crate::ir::Optimizer;
use crate::parser::Parser;
use crate::token::Token;
use crate::tokenizer::Tokenizer;

mod backends;
mod ir;
mod parser;
mod token;
mod tokenizer;

fn main() -> Result<(), ()> {
    let text = "";
    let tokens = Tokenizer::tokenize(&text);
    println!("{:?}", tokens);

    let expressions = Parser::parse(&tokens);
    println!("{:?}", expressions);

    let expressions = Optimizer::optimize(&expressions);
    println!("{:?}", expressions);

    let source_code = Transpiler::transpile(&expressions);
    println!("{}", source_code);

    Ok(())
}
