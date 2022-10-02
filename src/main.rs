mod lexer;
mod token;

use crate::lexer::Lexer;

fn main() {
    let text = "";
    let tokens = Lexer::tokenize(text);
}
