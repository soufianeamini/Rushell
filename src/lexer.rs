use std::{iter::Peekable, str::Chars};

use TokenType::*;

pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it = line.chars().peekable();
    let mut value = String::new();

    while let Some(char) = it.next() {
        match char {
            '|' => {
                if it.peek().is_some() && *it.peek().unwrap() == '|' {
                    generate_token("or", OR, &mut value, &mut tokens);
                    it.next();
                } else {
                    generate_token("pipe", PIPE, &mut value, &mut tokens);
                }
            }
            '<' => generate_token("input redirection", LESS, &mut value, &mut tokens),
            '>' => generate_token("output redirection", GREAT, &mut value, &mut tokens),
            ';' => generate_token("semicolon", SEMICOLON, &mut value, &mut tokens),
            ' ' => push_word(&mut value, &mut tokens),
            _ => value.push(char),
        }
    }

    push_word(&mut value, &mut tokens);
    tokens
}

// Ok so the way to refactor this, is to make a struct that contains the "happy" path for token
// and the repeat token names, and then you'd pass that in to the function, as well ass the char to
// check for, and the &mut value and &mut tokens obviously

#[derive(PartialEq, Debug)]
pub enum TokenType {
    WORD,
    PIPE,
    LESS,
    GREAT,
    SEMICOLON,
    OR,
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub ttype: TokenType,
}

impl Token {
    fn new(literal: &str, ttype: TokenType) -> Token {
        Token {
            literal: String::from(literal),
            ttype,
        }
    }
}

struct LexerOpt {
    single_literal: String,
    single_type: TokenType,
    repeat_literal: String,
    repeat_type: TokenType,
}

impl LexerOpt {
    fn new(l1: &str, t1: TokenType, l2: &str, t2: TokenType) -> LexerOpt {
        LexerOpt {
            single_literal: String::from(l1),
            single_type: t1,
            repeat_literal: String::from(l2),
            repeat_type: t2,
        }
    }
}

fn generate_repeatable_token(opt: LexerOpt, char_check: char, it: &mut Peekable<Chars> ,value: &mut String, tokens: &mut Vec<Token>) {
    if it.peek().is_some() && *it.peek().unwrap() == char_check {
        generate_token(&opt.repeat_literal, opt.repeat_type, value, tokens);
        it.next();
    } else {
        generate_token(&opt.single_literal, opt.single_type, value, tokens);
    }
}

fn generate_token(literal: &str, ttype: TokenType, value: &mut String, tokens: &mut Vec<Token>) {
    push_word(value, tokens);
    tokens.push(Token::new(literal, ttype));
}

fn push_word(value: &mut String, tokens: &mut Vec<Token>) {
    if !value.is_empty() {
        tokens.push(Token::new(value, WORD))
    }

    value.clear()
}
