use std::io::{self, Write};

#[derive(PartialEq)]
enum TokenType {
    WORD,
    PIPE,
    // AND,
    OR,
    LESS,
    LESSLESS,
    // GREAT,
    // GREATGREAT,
    // ERROR,
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
                if list.is_empty() {
                    println!("Error: syntax error near token `{}'", c);
                    return Vec::new();
                }
                let val = list.last().unwrap();
                if prevc == '|' && val.ttype == TokenType::PIPE {
                    list.pop();
                    list.push(Token::new(String::from("||"), TokenType::OR));
                } else if val.ttype != TokenType::WORD {
                            println!("Error: syntax error near token `{}'", val.value);
                            return Vec::new();
                } else {
                    list.push(Token::new(String::from("|"), TokenType::PIPE));
                }
                value = String::new();
            }
            '<' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                if let Some(val) = list.last() {
                    if prevc == '<' && val.ttype == TokenType::LESS {
                        list.pop();
                        list.push(Token::new(String::from("<<"), TokenType::LESSLESS));
                    } else if val.ttype != TokenType::WORD && val.ttype != TokenType::PIPE && val.ttype != TokenType::OR {
                                println!("Error: syntax error near token `{}'", val.value);
                                return Vec::new();
                    } else {
                        list.push(Token::new(String::from("<"), TokenType::LESS));
                    }
                } else {
                    list.push(Token::new(String::from("<"), TokenType::LESS))
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
    if !list.is_empty() {
        if list.last().unwrap().ttype != TokenType::WORD {
            println!("Error: syntax error near token `{}'", list.last().unwrap().value);
            return Vec::new();
        }
    }
    list
}

fn main() {
    loop {
        let mut line = String::new();

        print!("$ ");
        io::stdout().flush().expect("Error: Unable to flush buffer");
        io::stdin()
            .read_line(&mut line)
            .expect("Error: Unable to read from standard input");

        if line.is_empty() {
            return;
        }
        let list = lexer(&line);

        for token in list {
            print!("Token: {} -- Type: ", token.value);
            match token.ttype {
                TokenType::WORD => println!("Word"),
                TokenType::PIPE => println!("Pipe"),
                // TokenType::AND => println!("And"),
                TokenType::OR => println!("Or"),
                // TokenType::ERROR => println!("Error"),
                TokenType::LESS => println!("Input Redirection"),
                // TokenType::GREAT => println!("Output Redirection"),
                TokenType::LESSLESS => println!("Input Redirection - Append"),
                // TokenType::GREATGREAT => println!("Output Redirection - Append"),
            }
        }
    }
}
