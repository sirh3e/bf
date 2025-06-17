use crate::core::{
    ir::{Expression, optimizers::Optimizers},
    parser::Parser,
    tokenizer::Tokenizer,
};

pub struct Pipeline;

impl Pipeline {
    pub fn execute(text: &str) -> Vec<Expression> {
        let tokens = Tokenizer::tokenize(text);
        let expressions = Parser::parse(&tokens);

        Optimizers::optimize(&expressions)
    }
}
