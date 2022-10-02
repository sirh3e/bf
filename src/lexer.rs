use crate::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(text: &str) -> Vec<Token> {
        text.chars().into_iter().map(Self::tokenize_char).collect()
    }

    fn tokenize_char(char: char) -> Token {
        match char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '.' => Token::Dot,
            '>' => Token::Shr,
            '<' => Token::Shl,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            _ => Token::Whitespace,
        }
    }
}
