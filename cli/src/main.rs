// cli/src/main.rs

use clap::{Parser, Subcommand};
use rustyline::{Editor, error::ReadlineError};
use std::fs;
use std::io::Write;
mod lsp; 
use lsp::start_lsp;

// Ajouts indispensables pour résoudre les erreurs avec tower_lsp::async_trait
use std::option::Option;

use core::{parser::Parser as EdlParser, runtime::Interpreter};
use serde_json::{Value, json};

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
    /// Initialize a new EDL project
    Init,
    /// Live server system for EDL languages
    Lsp,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Run { file } => run_script(&file),
        Command::Repl => start_repl(),
        Command::Install { package } => install_package(&package),
        Command::Update { package } => update_package(&package),
        Command::List => list_packages(),
        Command::Init => init_project(),
        Command::Lsp => start_lsp().await?,
    }
    Ok(())
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
            Ok(_val) => {}
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

// --- Gestion des paquets ---

fn install_package(package: &str) {
    let url = format!("https://packages.quantum-os.org/edl/{}/latest", package);
    println!("📦 Downloading package from {url}");

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

                let manifest_path = "package.edl.json";
                let mut manifest: Value = if let Ok(data) = fs::read_to_string(manifest_path) {
                    serde_json::from_str(&data).unwrap_or_else(|_| json!({}))
                } else {
                    json!({})
                };

                if !manifest.get("dependencies").is_some() {
                    manifest["dependencies"] = json!({});
                }
                let deps = manifest.get_mut("dependencies")
                    .and_then(|d| d.as_object_mut())
                    .expect("dependencies should be an object");
                deps.insert(package.to_string(), Value::String("latest".to_string()));

                let manifest_str = serde_json::to_string_pretty(&manifest).unwrap();
                fs::write(manifest_path, manifest_str).expect("Failed to update package.edl.json");
                println!("🔗 Added '{}' to dependencies in package.edl.json", package);
            } else {
                eprintln!("❌ Package not found: {}", package);
            }
        }
        Err(e) => eprintln!("❌ Network error: {}", e),
    }
}

#[warn(unused_variables)]
fn update_package(package: &str) {
    let url = format!("https://packages.quantum-os.org/edl/update/{}/latest", package);
    println!("⬆️  Updating package '{}' from {url}", package);

    match reqwest::blocking::get(&url) {
        Ok(resp) => {
            if resp.status().is_success() {
                let code = resp.text().unwrap_or_default();
                let dir = "edl_modules";
                fs::create_dir_all(dir).ok();
                let path = format!("{}/{}.edl", dir, package);
                let mut file = fs::File::create(&path).expect("Failed to create module file");
                file.write_all(code.as_bytes()).expect("Failed to write module");
                println!("✅ Updated '{}'", package);
            } else {
                eprintln!("❌ Package not found or no update available: {}", package);
            }
        }
        Err(e) => eprintln!("❌ Network error: {}", e),
    }
}

fn list_packages() {
    let dir = "edl_modules";
    println!("📚 Installed packages:");
    match fs::read_dir(dir) {
        Ok(entries) => {
            let mut found = false;
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "edl" {
                        if let Some(name) = path.file_stem() {
                            println!(" - {}", name.to_string_lossy());
                            found = true;
                        }
                    }
                }
            }
            if !found {
                println!("(No packages installed)");
            }
        }
        Err(_) => {
            println!("(No packages installed)");
        }
    }
}

fn init_project() {
    if !std::path::Path::new("package.edl.json").exists() {
        let mut file = fs::File::create("package.edl.json").expect("Failed to create package.edl.json");
        let content = r#"{
    "name": "my-edl-project",
    "version": "0.1.0",
    "authors": ["Your Name"],
    "description": "A new EDL project",
    "scripts": {
        "build": "edl build",
        "test": "edl test"
    },
    "dependencies": {}
}
"#;
        file.write_all(content.as_bytes()).expect("Failed to write package.edl.json");
        println!("✅ Created package.edl.json");
    } else {
        println!("package.edl.json already exists.");
    }

    if !std::path::Path::new("edl_modules").exists() {
        fs::create_dir("edl_modules").expect("Failed to create edl_modules directory");
        println!("✅ Created edl_modules/");
    } else {
        println!("edl_modules/ already exists.");
    }

    println!("✨ EDL project initialized!");
}

fn run_script(script: &str) {
    let file = if script.ends_with(".edl") {
        script.to_string()
    } else {
        format!("{}.edl", script)
    };
    if std::path::Path::new(&file).exists() {
        run_file(&file);
        return;
    }

    let manifest_path = "package.edl.json";
    if let Ok(data) = fs::read_to_string(manifest_path) {
        if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(scripts) = manifest.get("scripts").and_then(|s| s.as_object()) {
                if let Some(cmd) = scripts.get(script).and_then(|v| v.as_str()) {
                    println!("▶️  Running script '{}' from package.edl.json: {}", script, cmd);
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .status()
                        .expect("Failed to run script");
                    return;
                }
            }
        }
    }

    eprintln!("❌ Script '{}' not found as a .edl file or in package.edl.json", script);
    std::process::exit(1);
}