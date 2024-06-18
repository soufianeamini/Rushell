use std::{error::Error, iter::Peekable, str::Chars};

use TokenType::*;

#[derive(Debug)]
pub enum TokenV2<'a> {
    Word(&'a str),
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
    Error(String),
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

pub fn lex_v2(line: &[u8]) -> Result<Vec<TokenV2>, Box<dyn Error>> {
    let mut tokens: Vec<TokenV2> = Vec::new();
    let mut it = line.iter().enumerate();
    let mut word_index = (false, 0);

    while let Some((i, char)) = it.next() {
        match char {
            b'&' | b'|' | b'<' | b'>' | b';' | b'(' | b')' | b' ' if word_index.0 => {
                let str_slice = std::str::from_utf8(&line[word_index.1..i])?;
                tokens.push(TokenV2::Word(str_slice));
                word_index.0 = false;
            }
            _ => (),
        }
        match char {
            b'&' => {
                if let Some(b'&') = line.get(i + 1) {
                    tokens.push(TokenV2::And);
                    it.next();
                } else {
                    tokens.push(TokenV2::Ampersand)
                }
            }
            b'|' => {
                if let Some(b'|') = line.get(i + 1) {
                    tokens.push(TokenV2::Or);
                    it.next();
                } else {
                    tokens.push(TokenV2::Pipe)
                }
            }
            b'<' => {
                if let Some(b'<') = line.get(i + 1) {
                    tokens.push(TokenV2::GreatGreat);
                    it.next();
                } else {
                    tokens.push(TokenV2::Great)
                }
            }
            b'>' => {
                if let Some(b'>') = line.get(i + 1) {
                    tokens.push(TokenV2::GreatGreat);
                    it.next();
                } else {
                    tokens.push(TokenV2::Great)
                }
            }
            b'"' | b'\'' => {
                unimplemented!()
            }
            b';' => tokens.push(TokenV2::Semicolon),
            b'(' => tokens.push(TokenV2::LeftParen),
            b')' => tokens.push(TokenV2::LeftParen),
            b' ' => (),
            _ => match word_index {
                (false, _) => word_index = (true, i),
                (true, _) => continue,
            },
        }
    }

    if word_index.0 {
        let str_slice = std::str::from_utf8(&line[word_index.1..])?;
        tokens.push(TokenV2::Word(str_slice));
    }
    Ok(tokens)
}

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
