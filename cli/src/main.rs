// cli/src/main.rs

use clap::{Parser, Subcommand};
use rustyline::{Editor, error::ReadlineError};
use std::fs;
use std::io::Write;
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
    /// Install an EDL package
    Install { package: String },
    /// Update an EDL package
    Update { package: String },
    /// List installed EDL packages
    List,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Run { file } => run_file(&file),
        Command::Repl => start_repl(),
        Command::Install { package } => install_package(&package),
        Command::Update { package } => update_package(&package),
        Command::List => list_packages(),
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
    let _ = rl.load_history("~/.edl_history");
    println!("✨ Welcome to the EDL REPL! Type ':help' or Ctrl+D to quit.");
    let mut line_num = 1;
    loop {
        let prompt = format!("\x1b[1;34medl\x1b[0m:{}> ", line_num);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line == ":exit" || line == "exit" {
                    println!("Goodbye!");
                    break;
                }
                if line == ":help" {
                    println!("EDL REPL commands:\n  :exit  Quit\n  :help  This help\n  :clear Clear screen");
                    continue;
                }
                if line == ":clear" {
                    print!("\x1b[2J\x1b[H");
                    continue;
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
                                        eprintln!("\x1b[1;31mRuntime error:\x1b[0m {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("\x1b[1;31mParse error:\x1b[0m {:?}", e),
                    }
                }
                line_num += 1;
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
    rl.append_history("~/.edl_history").ok();
}

// Ajoute des fonctions basiques pour l'instant
fn install_package(package: &str) {
    let url = format!("https://packages.quantum-os/edl/{}/latest", package);
    println!("📦 Downloading package from {url}");

    // Utilise reqwest pour télécharger le module (ajoute reqwest à Cargo.toml)
    match reqwest::blocking::get(&url) {
        Ok(resp) => {
            if resp.status().is_success() {
                let code = resp.text().unwrap_or_default();
                let dir = "edl_modules";
                fs::create_dir_all(dir).ok();
                let path = format!("{}/{}.edl", dir, package);
                let mut file = fs::File::create(&path).expect("Failed to create module file");
                file.write_all(code.as_bytes()).expect("Failed to write module");
                println!("✅ Installed '{}'", package);
            } else {
                eprintln!("❌ Package not found: {}", package);
            }
        }
        Err(e) => eprintln!("❌ Network error: {}", e),
    }
}

fn update_package(package: &str) {
    println!("⬆️  Updating package '{}'", package);
    // Ici tu pourrais mettre à jour le package
}

fn list_packages() {
    println!("📚 Installed packages:");
    // Ici tu pourrais lister les packages installés
}