use crate::core::{
    ir::{Expression, Optimizers},
    parser::Parser,
    tokenizer::Tokenizer,
};

pub struct Pipeline;

impl Pipeline {
    pub fn execute(text: &str) -> Vec<Expression> {
        let tokens = Tokenizer::tokenize(&text);
        let expressions = Parser::parse(&tokens);
        let expressions = Optimizers::optimize(&expressions);

        expressions
    }
}
