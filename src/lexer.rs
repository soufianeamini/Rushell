pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it = line.chars().peekable();
    let mut value = String::new();

    while let Some(char) = it.next() {
        match char {
            '|' => {
                push_word(&mut value, &mut tokens);
                tokens.push(Token {
                    literal: String::from("pipe"),
                    ttype: TokenType::PIPE,
                });
            }
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
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub ttype: TokenType,
}

fn push_word(value: &mut String, tokens: &mut Vec<Token>) {
    if !value.is_empty() {
        tokens.push(Token {
            literal: value.clone(),
            ttype: TokenType::WORD,
        })
    }

    value.clear()
}
