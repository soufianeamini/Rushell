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
