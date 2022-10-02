use crate::ir::Optimizer;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod ir;
mod lexer;
mod parser;
mod token;

fn main() {
    let text = "[[+][<++[-+]]]";

    let tokens = Lexer::tokenize(text);

    let expressions = Parser::parse(&tokens);
    println!("{:?}", expressions);

    let expressions = Optimizer::optimize(&expressions);
    println!("{:?}", expressions);
}
