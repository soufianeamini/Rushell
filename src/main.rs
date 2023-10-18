use rushell_improved::lexer;
use rustyline::DefaultEditor;
use std::process;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                let tokens = lexer::lex(&line);
                println!("{tokens:#?}");
            }
            Err(e) => match e.to_string().as_str() {
                "Interrupted" => continue,
                &_ => {
                    println!("{}", e);
                    process::exit(1)
                }
            },
        }
    }
}
