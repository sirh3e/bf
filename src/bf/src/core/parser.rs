use crate::core::{ir::Expression, token::Token};

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Vec<Expression> {
        let mut expressions = vec![];
        let mut indexes: Vec<usize> = vec![];

        for token in tokens.iter().filter(Self::filter) {
            match token {
                Token::Plus => {
                    expressions.push(Expression::IncVal(1));
                }
                Token::Minus => {
                    expressions.push(Expression::DecVal(1));
                }
                Token::Dot => {
                    expressions.push(Expression::Output);
                }
                Token::Comma => {
                    expressions.push(Expression::Input);
                }
                Token::Shr => {
                    expressions.push(Expression::IncPtr(1));
                }
                Token::Shl => {
                    expressions.push(Expression::DecPtr(1));
                }
                Token::OpenBracket => {
                    indexes.push(expressions.len());
                }
                Token::CloseBracket => {
                    let start_index = indexes.pop().unwrap();
                    let r#loop = expressions.split_off(start_index);
                    expressions.push(Expression::Loop(r#loop));
                }
                Token::Whitespace(_) => {
                    unreachable!()
                }
            }
        }
        expressions
    }

    fn filter(token: &&Token) -> bool {
        !matches!(token, Token::Whitespace(_))
    }
}
