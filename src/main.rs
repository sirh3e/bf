use crate::ir::Optimizer;
use crate::tokenizer::tokenizer;
use crate::parser::Parser;

mod ir;
mod tokenizer;
mod parser;
mod token;
mod backends;

fn main() {
    let text = "[[+][<++[-+]]]";

    let tokens = tokenizer::tokenize(text);

    let expressions = Parser::parse(&tokens);
    println!("{:?}", expressions);

    let expressions = Optimizer::optimize(&expressions);
    println!("{:?}", expressions);
}
