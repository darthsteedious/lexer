use std::str;
use std::i32;
use core::iter;

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
    let result: Vec<Token> = vec![];

    let mut char_iter = string.chars().peekable();
    let mut has_next = true;

    while has_next {
        if let Some(c) = char_iter.next() {
            let token = match c {
                ' ' | '\t' | '\n'  => parsers::parse_whitespace(&mut char_iter),
                chr @ '0'...'9'    => parsers::parse_number(chr, &mut char_iter),
                ';'                => Token::SemiColon,
                '('                => Token::OpenParen,
                ')'                => Token::CloseParen,
                '='                => parsers::parse_equals(&mut char_iter),
                u @ _              => parsers::parse_char(c, &mut char_iter)
            };
            println!("Got token {:?}", token);
        } else {
            has_next = false;
        }
    }

    result
}
