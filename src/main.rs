use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => println!("{line}"),
            Err(e) => {
                println!("{}", e.to_string());
                std::process::exit(1)
            }
        }
    }
}
