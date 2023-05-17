use std::fs::File;
use std::io::{self, Write};
use std::process;

#[derive(PartialEq,Debug)]
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

#[derive(Debug)]
struct Token {
    value: String,
    ttype: TokenType,
}

#[derive(Debug)]
struct Outfile {
    filename: String,
    append: bool,//false stands for truncate, true stands for append
}


#[derive(Debug)]
struct Heredoc {
    filename: String,
    limiter: String,//false stands for truncate, true stands for append
}
#[derive(Debug)]
struct Command {
    cmd: String,
    args: Vec<String>,//subject to change depending on what type the std::Command.args takes as
    //arguments
    infiles: Vec<String>,
    outfiles: Vec<Outfile>,
    heredocs: Vec<Heredoc>,
}

impl Command {
    fn new(cmd: String, args: Vec<String>, infiles: Vec<String>, outfiles: Vec<Outfile>, heredocs: Vec<Heredoc>) -> Command {
        Command {
            cmd,
            args,
            infiles,
            outfiles,
            heredocs,
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

impl Outfile {
    fn new(filename: String, append: bool) -> Outfile {
        Outfile {
            filename,
            append,
        }
    }
}

impl Heredoc {
    fn new(filename: String, limiter: String) -> Heredoc {
        Heredoc {
            filename,
            limiter,
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
                //I guess one solution is: You can use the value and look for a substr that starts
                //with $, and replace $expression with the new value. and then you can let the next
                //for append it to the list
                //
                //Also, it's fine if it recursively expands values that start with $
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

fn parser(list: &Vec<Token>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    let mut cmd = String::new();
    let mut args:Vec<String> = Vec::new();
    let mut infiles:Vec<String> = Vec::new();
    let mut outfiles:Vec<Outfile> = Vec::new();
    let mut heredocs:Vec<Heredoc> = Vec::new();
    let mut it = list.iter();
    while let Some(word) = it.next() {
        match word.ttype {
            TokenType::WORD => {
                if cmd.is_empty() {
                    cmd.clone_from(&word.value);
                } else {
                    args.push(word.value.clone());
                }
            }
            TokenType::LESS => {
                let word = it.nth(0).unwrap();
                infiles.push(word.value.clone());
            }
            TokenType::LESSLESS => {
                let word = it.nth(0).unwrap();
                heredocs.push(Heredoc::new(String::from("/tmp/TemporaryHeredocName"), word.value.clone()));
                infiles.push(String::from("/tmp/TemporaryHeredocName"))
            }
            TokenType::GREAT => {
                let word = it.nth(0).unwrap();
                outfiles.push(Outfile::new(word.value.clone(), false));
            }
            TokenType::GREATGREAT => {
                let word = it.nth(0).unwrap();
                outfiles.push(Outfile::new(word.value.clone(), true));
            }
            TokenType::PIPE => {
                commands.push(
                    Command::new(cmd, args, infiles, outfiles, heredocs)
                );
                cmd = String::new();
                args = Vec::new();
                infiles = Vec::new();
                outfiles = Vec::new();
                heredocs = Vec::new();
            }
            _ => panic!("Unhandled Token Type"),
        }
    }
    commands.push(
        Command::new(cmd, args, infiles, outfiles, heredocs)
    );
    commands
}

fn execute_commands(list: &Vec<Command>) {
    let mut proc: Vec<process::Child> = Vec::new();
    let mut it = list.iter();
    let mut prevstdout: Option<process::ChildStdout> = None;
    let mut first = true;

    while let Some(command) = it.next() {
        let mut child = process::Command::new(&command.cmd);
        let child = child.args(&command.args);
        if first == false {
            if prevstdout.is_some() {
                child.stdin(prevstdout.unwrap());
            } else {
                child.stdin(process::Stdio::null());
            }
        }
        first = false;
        if it.len() != 0 {
            child.stdout(process::Stdio::piped());
        }

        // Working, but needs to handle pipes better before enabling outfiles
        //
        let mut itoutfile = command.outfiles.iter();
        while let Some(out) = itoutfile.next() {
            let file = File::create(&out.filename).unwrap();
            child.stdout(process::Stdio::from(file));
        }

        let mut spawn = match child.spawn() {
            Ok(spawn) => spawn,
            Err(_) => {
                eprintln!("Error: {}: command not found", command.cmd);
                prevstdout = None;
                continue;
            }
        };
        prevstdout = spawn.stdout.take();
        proc.push(spawn);
    }
    for i in 0..proc.len() {
        proc.get_mut(i).unwrap().wait().unwrap();
    }
}

fn main() {
    loop {
        let mut line = String::new();

        print!("$ ");
        io::stdout().flush().expect("Error: Unable to flush buffer");
        io::stdin()
            .read_line(&mut line)
            .expect("Error: Unable to read from standard input");

        if line.trim().is_empty() {
            continue;
        }
        let list = lexer(&line);
        if list.is_empty() {
            continue;
        }
        // print_tokens(&list);
        let cmds = parser(&list);
        execute_commands(&cmds);
        // dbg!(&cmds);
    }
}
