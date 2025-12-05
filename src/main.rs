use clap::Parser;
use anyhow::{ Result };
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::io::stdin;
use std::io::Write;
use std::io;
use std::process::exit;
use wordnet;
use pyo3::prelude::*;
mod token;
mod scanner;
mod token_type;

#[derive(Parser, Default, Debug)]
#[command(name = "qed")]
struct Args {
    #[arg(short, long, default_value = None)]
    /// [Optional] path to the file to run
    filepath: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Some(path) = args.filepath {
        let content = fs::read_to_string(&path).expect("Could not read query.");
        run(&content);
        println!("");
        run_prompt();
    } else {
        run_prompt();
    }

    Ok(())
}

fn get_sentences(content: &String) -> Vec<HashMap<String, String>> {

    Python::attach(|py| {
        let nlp_api = PyModule::import(py, "nlp_api").expect("nlp_api import failed.");
        let sentences: Vec<HashMap<String, String>> = nlp_api
            .getattr("get_token_info").expect("fetching get_token_info function failed.")
            .call1((content,)).expect("function call to get_token_info failed.")
            .extract().expect("token deconstruction failed.");

        sentences
    })
}

fn run(content: &String) {
    let sentences = get_sentences(content);
    println!("{:?}", sentences);

    //let wn = wordnet::Database::open(&::std::path::Path::new("./wordnet")).unwrap();
}

fn run_prompt() {
    loop {
        let mut input: String = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Exiting..."); //TODO: refine user input functionality
        run(&input);
    }
}
