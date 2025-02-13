use clap::Parser;
use std::io;
use std::io::Error;
use std::io::IsTerminal;
use std::io::prelude::*;
use bagelwithlox::source::Source;
use bagelwithlox::interpreter::Interpreter;

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


fn read_stdin_line(input: &mut String) -> Result<(), Error> {
    input.clear();
    io::stdout().write(b"bwl > ")?;
    io::stdout().flush()?;
    match io::stdin().read_line(input) {
        Err(e) => Err(e),
        _ => Ok(()),
    }
}


fn repl(interpreter: &mut Interpreter) -> Result<(), String> {
    eprintln!("Running the repl!");
    let mut input = String::new();

    loop {
        match read_stdin_line(&mut input) {
            Err(e) => return Err(format!("Failed to read stdin: {}", e)),
            _ => (),
        }

        if input == "" { return Ok(()); }
        if input == "\n" { continue; }


        match interpreter.interpret(
            &mut Source::from_string(input.to_string()),
        ) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
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
