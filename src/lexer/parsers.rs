use std::str;
use std::i32;
use core::iter;

use super::Token;
use super::Op;

pub fn parse_whitespace(iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut peek_whitespace = true;

    while peek_whitespace {
        peek_whitespace = if let Some(c) = iter.peek() {
            c.is_whitespace() 
        } else { false };


        if peek_whitespace {
            iter.next();    
        }    
    } 

    Token::Whitespace
}

pub fn parse_number(chr: char, iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut peek_num = true;
    let mut result_string: String = String::new();

    result_string.push(chr);

    while peek_num {
        if let Some(c) = iter.peek() {                    
            if c.is_digit(10) {                                      
                result_string.push(*c);
                peek_num = true; 
            } 
            else { peek_num = false; }
        }             

        if peek_num {
            iter.next();
        }
    }

    let number = match i32::from_str_radix(result_string.as_str(), 10) {
        Ok(int) => int,
        Err(_) => abort("Error parsing int."),
    };

    Token::Number(number)
}

fn is_identifier_end(c: &char) -> bool {
    vec!['(', ')', ';'].iter().any(|x| x.eq(c))
}


pub fn parse_char(chr: char, iter: &mut iter::Peekable<str::Chars>) -> Token {
    let keywords = vec!["if", "else"];
    let mut peek_char = true;
    let mut result_string: String = String::new();

    result_string.push(chr);

    while peek_char {
        if let Some(c) = iter.peek() {
            peek_char = !c.is_whitespace() && !is_identifier_end(&c);
            
            if peek_char {
                result_string.push(*c);
            }
        } else { peek_char = false; }
                        
        if peek_char { iter.next(); }
    }

    let res = result_string.as_str();

    if keywords.iter().any(|x| x.eq(&res)) { Token::Keyword(res.to_string()) }
    else { Token::Identifier(res.to_string()) }
}

pub fn parse_equals(iter: &mut iter::Peekable<str::Chars>) -> Token {
    let mut is_assign = true;

    if let Some(c) = iter.peek() {
        is_assign = *c != '=';
    }
    
    if !is_assign { iter.next(); }

    if is_assign { Token::Operator(Op::Assign) }
    else { Token::Operator(Op::Equals) }
}

fn abort<TResult>(description: &'static str) -> TResult {
    panic!(description);
}
