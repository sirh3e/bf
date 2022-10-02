use crate::token::Token;

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> () {
        let d = tokens.into_iter().filter(Self::filter);
    }

    fn filter(token: &&Token) -> bool {
        match &token {
            Token::Whitespace => true,
            _ => false,
        }
    }
}
