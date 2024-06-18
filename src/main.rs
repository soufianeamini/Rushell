use rushell_improved::lexer;
use rustyline::DefaultEditor;
use std::process;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                rl.add_history_entry(line.clone()).unwrap_or_else(|_| {
                    eprintln!("WARNING: couldn't add command to history");
                    false
                });
                let line = line.trim();
                let tokens_v2 = lexer::lex(line.as_bytes());
                println!("{tokens_v2:#?}");
            }
            Err(e) => match e.to_string().as_str() {
                "Interrupted" => continue,
                &_ => {
                    println!("{e}");
                    process::exit(1)
                }
            },
        }
    }
}
