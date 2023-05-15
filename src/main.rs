use std::io::{self, Write};

#[derive(PartialEq)]
enum TokenType {
    WORD,
    PIPE,
    AND,
    OR,
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
    let mut prevc = ' ';
    for c in line.trim().chars() {
        match c {
            '|' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                if prevc == '|' {
                    if let Some(val) = list.last() {
                        if val.ttype != TokenType::PIPE {
                            println!("Error: syntax error near token `{}'", val.value);
                            return Vec::new();
                        }
                    }
                    list.pop();
                    list.push(Token::new(String::from("||"), TokenType::OR));
                    //then add and else if last token is different from word then error as well
                } else {
                    list.push(Token::new(String::from("|"), TokenType::PIPE));
                }
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
        prevc = c;
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
            TokenType::AND => println!("And"),
            TokenType::OR => println!("Or"),
            TokenType::INFILE => println!("Infile"),
            TokenType::OUTFILE => println!("Outfile"),
        }
    }
}
