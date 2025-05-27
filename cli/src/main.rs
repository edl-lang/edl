// edl-cli/src/main.rs

use clap::{Parser, Subcommand};
use rustyline::Editor;
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
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    let mut parser = EdlParser::new(&code);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    };
    let mut interp = Interpreter::new();
    for stmt in stmts {
        match interp.eval_stmt(&stmt) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Runtime error: {:?}", e);
                break;
            }
        }
    }
}

fn start_repl() {
    let mut rl = Editor::<()>::new().unwrap();
    let mut interp = Interpreter::new();
    println!("EDL REPL. Type 'exit' or Ctrl+D to quit.");
    loop {
        let readline = rl.readline("edl> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit" { break; }
                rl.add_history_entry(line.as_str());
                let mut parser = EdlParser::new(&line);
                match parser.parse() {
                    Ok(stmts) => {
                        for stmt in stmts {
                            match interp.eval_stmt(&stmt) {
                                Ok(val) => if let core::runtime::Value::Null = val {
                                    // don't print Null
                                } else {
                                    println!("{:?}", val);
                                },
                                Err(e) => println!("Runtime error: {:?}", e),
                            }
                        }
                    },
                    Err(e) => println!("Parse error: {:?}", e),
                }
            },
            Err(_) => break,
        }
    }
}