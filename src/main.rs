use clap::Parser;
use bagelwithlox::reader;
use bagelwithlox::interpreter::Interpreter;

#[derive(Parser)]
#[command(
    version,
    about,
)]
struct Cli {}


fn main() {
    let _cli = Cli::parse();

    let src = reader::read_source("file.bwl");
    Interpreter::new().interpret(src);
}
