use std::io::{self, Write};

#[derive(PartialEq)]
enum TokenType {
    WORD,
    PIPE,
    // AND,
    OR,
    LESS,
    LESSLESS,
    GREAT,
    GREATGREAT,
    // ERROR,
}

struct Token {
    value: String,
    ttype: TokenType,
}

struct Command {
    cmd: String,
    args: Vec<String>,//subject to change depending on what type the std::Command.args takes as
    //arguments
    infiles: Vec<String>,
    outfiles: Vec<String>,
}

impl Command {
    fn new(cmd: String, args: Vec<String>, infiles: Vec<String>, outfiles: Vec<String>) -> Command {
        Command {
            cmd,
            args,
            infiles,
            outfiles,
        }
    }
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
    let mut doublequotes = false;
    let mut singlequotes = false;
    for c in line.trim().chars() {
        if doublequotes == true {
            if c == '"' {
                doublequotes = false;
                //If you use this logic, you would have to expand variables inside the lexer...
            } else {
                value.push(c);
            }
            continue;
        }
        if singlequotes == true {
            if c == '\'' {
                singlequotes = false;
            } else {
                value.push(c);
            }
            continue;
        }
        match c {
            '"' => doublequotes = true,
            '\'' => singlequotes = true,
            '|' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                if list.is_empty() {
                    unexpected_token(&c.to_string());
                    return Vec::new();
                }
                let val = list.last().unwrap();
                if prevc == '|' && val.ttype == TokenType::PIPE {
                    list.pop();
                    list.push(Token::new(String::from("||"), TokenType::OR));
                } else if val.ttype != TokenType::WORD {
                    unexpected_token(&val.value);
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
                        unexpected_token(&val.value);
                        return Vec::new();
                    } else {
                        list.push(Token::new(String::from("<"), TokenType::LESS));
                    }
                } else {
                    list.push(Token::new(String::from("<"), TokenType::LESS))
                }
                value = String::new();
            }
            '>' => {
                if !value.is_empty() {
                    list.push(Token::new(value, TokenType::WORD));
                }
                if let Some(val) = list.last() {
                    if prevc == '>' && val.ttype == TokenType::GREAT {
                        list.pop();
                        list.push(Token::new(String::from(">>"), TokenType::GREATGREAT));
                    } else if val.ttype != TokenType::WORD && val.ttype != TokenType::PIPE && val.ttype != TokenType::OR {
                        unexpected_token(&val.value);
                        return Vec::new();
                    } else {
                        list.push(Token::new(String::from(">"), TokenType::GREAT));
                    }
                } else {
                    list.push(Token::new(String::from(">"), TokenType::GREAT))
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
    if !list.is_empty() && list.last().unwrap().ttype != TokenType::WORD {
        unexpected_token(&list.last().unwrap().value);
        return Vec::new();
    }
    if doublequotes == true {
        println!("Error: unclosed quotes");
        return Vec::new();
    }
    list
}

fn unexpected_token(s:&String) {
    println!("Error: syntax error near unexpected token `{}'", s);
}

fn print_tokens(list: &Vec<Token>) {
    for token in list {
        print!("Token: {} -- Type: ", token.value);
        match token.ttype {
            TokenType::WORD => println!("Word"),
            TokenType::PIPE => println!("Pipe"),
            // TokenType::AND => println!("And"),
            TokenType::OR => println!("Or"),
            // TokenType::ERROR => println!("Error"),
            TokenType::LESS => println!("Input Redirection"),
            TokenType::GREAT => println!("Output Redirection - Truncate"),
            TokenType::LESSLESS => println!("Heredoc"),
            TokenType::GREATGREAT => println!("Output Redirection - Append"),
        }
    }
}

// fn parser(list: &Vec<Token>) {
//     let mut cmd = String::new();
//     let mut args:Vec<String> = Vec::new();
//     for word in list {
//         match word.ttype {
//             TokenType::WORD => {
//                 if cmd.is_empty() {
//                     cmd.clone_from(&word.value);
//                 } else {
//                     args.push(word.value.clone());
//                 }
//             }
//         }
//     }
// }

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
        print_tokens(&list);//Try adding handling of quotes now, before going to parsing (which
        //handles env variables)
        // parser(&list);
    }
}
