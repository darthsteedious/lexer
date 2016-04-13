mod parsers;

use self::parsers::{parse_whitespace, parse_number, parse_equals, parse_char};


#[derive(Debug)]
pub enum Op {
    Assign,
    Equals,
    Add,
    Subtract,
    Divide,
    Multiply,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug)]
pub enum Token {
    Unknown(char),
    Whitespace,
    Keyword(String),
    Identifier(String),
    Number(i32),
    OpenParen,
    CloseParen,
    SemiColon,
    Operator(Op),
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let mut char_iter = string.chars().peekable();
    let mut has_next = true;

    while has_next {
        if let Some(c) = char_iter.next() {
            let token = match c {
                ' ' | '\t' | '\n'  => parse_whitespace(&mut char_iter),
                chr @ '0'...'9'    => parse_number(chr, &mut char_iter),
                ';'                => Token::SemiColon,
                '('                => Token::OpenParen,
                ')'                => Token::CloseParen,
                '='                => parse_equals(&mut char_iter),
                 _                 => parse_char(c, &mut char_iter)
            };

            println!("Got token {:?}", token);
            result.push(token);
        } else {
            has_next = false;
        }
    }

    result
}
