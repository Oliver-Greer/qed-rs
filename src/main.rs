use clap::Parser;
use anyhow::{ Context, Result };
use std::fs;
use std::path::PathBuf;
use std::io::stdin;
mod token;
mod scanner;
mod token_type;
use token::Token;
use scanner::Scanner;

#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long, default_value = None)]
    /// [Optional] path to the file to run
    filepath: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();

    if let Some(path) = args.filepath {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Could not read file `{}`", path.display()));
        run(content.unwrap());
    } else {
        run_prompt();
    }

    Ok(())
}

fn run(content: String) {
    let mut scanner: Scanner = Scanner::new(content);
    scanner.scan_tokens();
}

fn run_prompt() {
    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Exiting...");
        run(input);
    }
}
