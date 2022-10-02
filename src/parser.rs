use crate::ir::Expression;
use crate::token::Token;

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Vec<Expression> {
        let mut index = 0;
        let mut expressions = vec![];
        let mut indexes = vec![];

        for token in tokens.into_iter().filter(Self::filter) {
            match token {
                Token::Plus => {
                    index += 1;
                    expressions.push(Expression::IncVal(1));
                }
                Token::Minus => {
                    index += 1;
                    expressions.push(Expression::DecVal(1));
                }
                Token::Dot => {
                    index += 1;
                    expressions.push(Expression::Output);
                }
                Token::Comma => {
                    index += 1;
                    expressions.push(Expression::Input);
                }
                Token::Shr => {
                    index += 1;
                    expressions.push(Expression::IncPtr(1));
                }
                Token::Shl => {
                    index += 1;
                    expressions.push(Expression::DecPtr(1));
                }
                Token::OpenBracket => {
                    indexes.push(index);
                }
                Token::CloseBracket => {
                    let start_index = indexes.pop().unwrap();
                    let r#loop = expressions.split_off(start_index);
                    expressions.push(Expression::Loop(r#loop));
                }
                Token::Whitespace => {
                    unreachable!()
                }
            }
        }
        expressions
    }

    fn filter(token: &&Token) -> bool {
        match &token {
            Token::Whitespace => false,
            _ => true,
        }
    }
}
