use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod token;

fn main() {
    let text = "";
    let tokens = Lexer::tokenize(text);
    let unit = Parser::parse(&tokens);
}
