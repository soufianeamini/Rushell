use core::fmt;
use std::error::Error;

#[derive(Debug)]
struct LexerError(String);

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum Token<'a> {
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
}

pub fn lex(line: &[u8]) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it = line.iter().enumerate();
    let mut word_index = (false, 0);

    while let Some((i, char)) = it.next() {
        match char {
            b'&' | b'|' | b'<' | b'>' | b';' | b'(' | b')' | b' ' if word_index.0 => {
                let str_slice = std::str::from_utf8(&line[word_index.1..i])?;
                tokens.push(Token::Word(str_slice));
                word_index.0 = false;
            }
            _ => (),
        }
        match char {
            b'&' => {
                if let Some(b'&') = line.get(i + 1) {
                    tokens.push(Token::And);
                    it.next();
                } else {
                    tokens.push(Token::Ampersand)
                }
            }
            b'|' => {
                if let Some(b'|') = line.get(i + 1) {
                    tokens.push(Token::Or);
                    it.next();
                } else {
                    tokens.push(Token::Pipe)
                }
            }
            b'<' => {
                if let Some(b'<') = line.get(i + 1) {
                    tokens.push(Token::LessLess);
                    it.next();
                } else {
                    tokens.push(Token::Less)
                }
            }
            b'>' => {
                if let Some(b'>') = line.get(i + 1) {
                    tokens.push(Token::GreatGreat);
                    it.next();
                } else {
                    tokens.push(Token::Great)
                }
            }
            b'"' | b'\'' => {
                if let (false, _) = word_index {
                    word_index = (true, i)
                }

                let error_handler = || Box::new(LexerError(String::from("Unclosed Quotes")));
                it.find(|p| p.1 == char).ok_or_else(error_handler)?;
            }
            b';' => tokens.push(Token::Semicolon),
            b'(' => tokens.push(Token::LeftParen),
            b')' => tokens.push(Token::RightParen),
            b' ' => (),
            _ => match word_index {
                (false, _) => word_index = (true, i),
                (true, _) => continue,
            },
        }
    }

    if word_index.0 {
        let str_slice = std::str::from_utf8(&line[word_index.1..])?;
        tokens.push(Token::Word(str_slice));
    }
    Ok(tokens)
}

