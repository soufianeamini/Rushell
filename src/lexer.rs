use std::{iter::Peekable, str::Chars};

use TokenType::*;

pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it = line.chars().peekable();
    let mut value = String::new();

    while let Some(char) = it.next() {
        match char {
            '&' => {
                let opt = LexerOpt::new("background token", Ampersand, "and", And);
                generate_repeatable_token(opt, '&', &mut it, &mut value, &mut tokens);
            }
            '|' => {
                let opt = LexerOpt::new("pipe", Pipe, "or", Or);
                generate_repeatable_token(opt, '|', &mut it, &mut value, &mut tokens);
            }
            '<' => {
                let opt = LexerOpt::new("input redirection", Less, "heredoc", LessLess);
                generate_repeatable_token(opt, '<', &mut it, &mut value, &mut tokens);
            }
            '>' => {
                let opt = LexerOpt::new(
                    "output redirection",
                    Great,
                    "output redirection - append",
                    GreatGreat,
                );
                generate_repeatable_token(opt, '>', &mut it, &mut value, &mut tokens);
            }
            '"' | '\'' => {
                value.push(char);
                for c in it.by_ref() {
                    value.push(c);
                    if c == char {
                        break;
                    }
                }
                if it.peek().is_none() && !value.ends_with(char) {
                    tokens.clear();
                    tokens.push(Token::new("Unclosed quotes", Error));
                    return tokens;
                }
            }
            ';' => generate_token("semicolon", Semicolon, &mut value, &mut tokens),
            '(' => generate_token("left parentheses", LeftParen, &mut value, &mut tokens),
            ')' => generate_token("right parentheses", RightParen, &mut value, &mut tokens),
            ' ' => push_word(&mut value, &mut tokens),
            _ => value.push(char),
        }
    }

    push_word(&mut value, &mut tokens);
    tokens
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Word,
    Pipe,
    Ampersand,
    Less,
    Great,
    LessLess,
    GreatGreat,
    Semicolon,
    Or,
    And,
    LeftParen,
    RightParen,
    Error,
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
    fn new(
        single_literal: &str,
        single_type: TokenType,
        repeat_literal: &str,
        repeat_type: TokenType,
    ) -> LexerOpt {
        LexerOpt {
            single_literal: String::from(single_literal),
            single_type,
            repeat_literal: String::from(repeat_literal),
            repeat_type,
        }
    }
}

fn generate_repeatable_token(
    opt: LexerOpt,
    char_check: char,
    it: &mut Peekable<Chars>,
    value: &mut String,
    tokens: &mut Vec<Token>,
) {
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
        tokens.push(Token::new(value, Word))
    }

    value.clear()
}
