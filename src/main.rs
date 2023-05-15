use std::io::{self, Write};

enum TokenType {
    WORD,
    PIPE,
    OP,
    INFILE,
    OUTFILE,
}

struct Token {
    value: String,
    ttype: TokenType,
}

impl Token {
    fn new(value: String, ttype: TokenType) -> Token {
        Token {
            value,
            ttype,
        }
    }
}

fn lexer(line: &String) -> Vec<Token> {
    let mut list: Vec<Token> = Vec::new();
    let mut value = String::new();
    for c in line.trim().chars() {
        match c {
            '|' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                list.push(Token::new(String::from("|"), TokenType::PIPE));
                value = String::new();
            }
            ' ' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                value = String::new();
            }
            _ => {
                value.push(c);
            }
        }
    }
    if !value.is_empty() {
        list.push(Token::new(value, TokenType::WORD));
    }
    list
}

fn main() {
    let mut line = String::new();

    print!("> ");
    io::stdout().flush().expect("Error: Unable to flush buffer");
    io::stdin()
        .read_line(&mut line)
        .expect("Error: Unable to read from standard input");

    let list = lexer(&line);

    for token in list {
        print!("Token: {} -- Type: ", token.value);
        match token.ttype {
            TokenType::WORD => println!("Word"),
            TokenType::PIPE => println!("Pipe"),
            TokenType::OP => println!("Operator"),
            TokenType::INFILE => println!("Infile"),
            TokenType::OUTFILE => println!("Outfile"),
        }
    }
}
