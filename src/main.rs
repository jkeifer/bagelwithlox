use clap::Parser;
use std::io;
use std::io::Error;
use std::io::IsTerminal;
use bagelwithlox::reader::Source;
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
    fn get_source(&self) -> Option<Result<Source, Error>> {

        if let Some(cmd) = &self.cmd {
            eprintln!("Reading from command option");
            return Some(Ok(Source::from_string(&cmd)));
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
            Err(e) => return Some(Err(e)),
        };

        return Some(Ok(Source::from_string(&stdin)));
    }
}


fn repl() {
    eprintln!("Running the repl!");
}


fn main() {
    let cli = Cli::parse();
    let interpreter =  Interpreter::new();

    if let Some(src) = cli.get_source() {
        match src {
            Ok(src) => {
                eprintln!("Got the following source content:\n'''\n{}\n'''", &src.get_content());
                interpreter.interpret(src)
            },
            Err(e) => eprintln!("Encountered an error: {}", e),
        }
    } else {
        repl()
    }
}
