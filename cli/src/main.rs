// cli/src/main.rs

use clap::{Parser, Subcommand};
use rustyline::{Editor, error::ReadlineError};
use std::fs;
use core::{parser::Parser as EdlParser, runtime::Interpreter};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a .edl script file
    Run { file: String },
    /// Start the EDL REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Run { file } => run_file(&file),
        Command::Repl => start_repl(),
    }
}

fn run_file(file: &str) {
    let code = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read file '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    let mut parser = EdlParser::new(&code);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Parse error in file '{}': {:?}", file, e);
            std::process::exit(1);
        }
    };

    let mut interp = Interpreter::new();

    for stmt in stmts {
        match interp.eval_stmt(&stmt) {
            Ok(_val) => {
                // Ne rien afficher ici : print() est déjà géré dans le runtime
            }
            Err(e) => {
                eprintln!("❌ Runtime error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
}

fn start_repl() {
    let mut rl = Editor::<()>::new().unwrap();
    let mut interp = Interpreter::new();
    println!("✨ Welcome to the EDL REPL! Type 'exit' or Ctrl+D to quit.");
    loop {
        let readline = rl.readline("edl> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line == "exit" {
                    println!("Goodbye!");
                    break;
                }
                if !line.is_empty() {
                    rl.add_history_entry(line);
                    let mut parser = EdlParser::new(line);
                    match parser.parse() {
                        Ok(stmts) => {
                            for stmt in stmts {
                                match interp.eval_stmt(&stmt) {
                                    Ok(val) => {
                                        if let core::runtime::Value::Null = val {
                                            // don't print Null
                                        } else {
                                            println!("{:?}", val);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Runtime error: {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("Parse error: {:?}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\nGoodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        }
    }
}