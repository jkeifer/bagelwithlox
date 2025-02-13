use clap::Parser;
use std::io;
use std::io::IsTerminal;
use bagelwithlox::source::Source;
use bagelwithlox::interpreter::Interpreter;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RLResult};

#[derive(Parser)]
#[command(
    version,
    about,
)]
struct Cli {
    #[arg(short)]
    cmd: Option<String>,
    file: Option<String>,
}

impl Cli {
    fn get_source(&self) -> Option<Result<Source, String>> {

        if let Some(cmd) = &self.cmd {
            eprintln!("Reading from command option");
            return Some(Ok(Source::from_string(cmd.to_string())));
        }

        if let Some(path) = &self.file {
            eprintln!("Reading from file arg '{}'", &path);
            return Some(Source::from_file(&path));
        }

        if io::stdin().is_terminal() {
            return None;
        }

        eprintln!("Reading from stdin");
        let stdin = match io::read_to_string(io::stdin()) {
            Ok(string) => string,
            Err(e) => return Some(Err(format!("Failed to read stdin: {}", e))),
        };

        return Some(Ok(Source::from_string(stdin)));
    }
}


fn repl(interpreter: &mut Interpreter) -> RLResult<()> {
    eprintln!("Running the repl!");
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("bwl >");
        match readline {
            Ok(line) => {
                if line.trim() == "" { continue; }
                rl.add_history_entry(line.as_str())?;
                match interpreter.interpret(
                    &mut Source::from_string(line.to_string()),
                ) {
                    Ok(Some(result)) => println!("{}", result),
                    Ok(None) => (),
                    Err(e) => eprintln!("{}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}


fn main() {
    let cli = Cli::parse();
    let mut interpreter =  Interpreter::new();

    if let Some(src) = cli.get_source() {
        match src {
            Ok(mut src) => {
                eprintln!("Got the following source content:\n'''\n{}\n'''", &src.content);
                if let Err(e) = interpreter.interpret(&mut src) {
                    eprintln!("ERROR: {}", e)
                }
            },
            Err(e) => eprintln!("ERROR: {}", e),
        }
    } else {
        match repl(&mut interpreter) {
            Err(e) => eprintln!("ERROR: {}", e),
            _ => (),
        }
    }
}
