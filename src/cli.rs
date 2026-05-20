use std::fs;
use std::io::{self, Write};
use std::process::Command;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::sema::SemanticAnalyzer;

pub fn execute_subcommand(args: &[String]) {
    // If no arguments are passed, open the interactive REPL (IDLE)
    if args.is_empty() {
        start_repl();
        return;
    }

    let command = &args[0];
    match command.as_str() {
        "build" => {
            if args.len() < 2 {
                eprintln!("Error: Please provide a source file. (e.g., krait build src/main.kr)");
                return;
            }
            let _ = compile_file(&args[1]);
        }
        "run" => {
            if args.len() < 2 {
                eprintln!("Error: Please provide a source file. (e.g., krait run src/main.kr)");
                return;
            }
            let _ = run_file(&args[1]);
        }
        "new" => {
            if args.len() < 2 {
                eprintln!("Error: Please provide a project name. (e.g., krait new my_app)");
                return;
            }
            create_project(&args[1]);
        }
        "check" => {
            if args.len() < 2 {
                eprintln!("Error: Please provide a source file. (e.g., krait check src/main.kr)");
                return;
            }
            let _ = check_file(&args[1]);
        }
        "format" => {
            if args.len() < 2 {
                eprintln!("Error: Please provide a source file. (e.g., krait format src/main.kr)");
                return;
            }
            let _ = format_file(&args[1]);
        }
        "-v" | "--version" => {
            println!("Krait v0.1.0 (Production Engine)");
        }
        _ => {
            eprintln!("Unknown command option: {}", command);
            print_help();
        }
    }
}

// ---------------------------------------------------------
// Phase 3: The Interactive REPL (IDLE)
// ---------------------------------------------------------
fn start_repl() {
    println!("Krait 0.1.0 Interactive Shell");
    println!("Type 'exit' to quit.\n");
    
    let mut interpreter = crate::interpreter::Interpreter::new();
    let mut sema = SemanticAnalyzer::new(); 

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end().to_string() + "\n"; // Ensure newline for the parser
        
        if input.trim() == "exit" { break; }
        if input.trim().is_empty() { continue; }

        match Lexer::tokenize(&input) {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse_program() {
                    Ok(ast) => {
                        let _ = sema.analyze(&ast); // Ignore type errors in REPL to be flexible
                        match interpreter.interpret(&ast) {
                            Ok(Some(val)) => match val { // Only print values that aren't Void
                                crate::interpreter::Value::Void => {},
                                crate::interpreter::Value::Int(v) => println!("{}", v),
                                crate::interpreter::Value::Float(v) => println!("{}", v),
                                crate::interpreter::Value::Str(v) => println!("\"{}\"", v),
                                crate::interpreter::Value::Bool(v) => println!("{}", v),
                            },
                            Ok(None) => {}, // Standard statements yield nothing visually
                            Err(e) => eprintln!("Runtime Error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Parse Error: {}", e),
                }
            }
            Err(e) => eprintln!("Syntax Error: {}", e),
        }
    }
}

// ---------------------------------------------------------
// Phase 3: Package System Scaffolding (`krait new`)
// ---------------------------------------------------------
fn create_project(name: &str) {
    if fs::create_dir(name).is_err() {
        eprintln!("Error: Directory '{}' already exists.", name);
        return;
    }
    
    let toml_content = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\ncompiler = \">=0.1.0\"\n\n[dependencies]\n# Add packages here\n", 
        name
    );
    fs::write(format!("{}/krait.toml", name), toml_content).unwrap();
    
    fs::create_dir(format!("{}/src", name)).unwrap();
    let main_kr_content = "make main()\n    show \"Hello from Krait!\"\n\nmain()\n";
    fs::write(format!("{}/src/main.kr", name), main_kr_content).unwrap();

    println!("Created production Krait project `{}`.", name);
    println!("  cd {}", name);
    println!("  krait run src/main.kr");
}

// ---------------------------------------------------------
// Standard Commands
// ---------------------------------------------------------
fn print_help() {
    println!("Krait Language Compiler Interface");
    println!("Usage:");
    println!("  krait [COMMAND] [FILE]");
    println!("\nCommands:");
    println!("  (no args)      Start the Interactive REPL shell");
    println!("  new    <name>  Initialize a new Krait project directory");
    println!("  build  <file>  Compiles file to Native Executable (Requires Clang)");
    println!("  run    <file>  Executes script immediately via tree-walking interpreter");
    println!("  check  <file>  Executes frontend syntactic correctness validation passes");
    println!("  format <file>  Applies standard spacing and style normalization directly to disk");
}

fn compile_file(path: &str) -> Result<(), ()> {
    let source = fs::read_to_string(path).map_err(|e| eprintln!("I/O Error: {}", e))?;
    let tokens = Lexer::tokenize(&source).map_err(|e| eprintln!("Syntax: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().map_err(|e| eprintln!("Parse: {}", e))?;
    
    let mut sema = SemanticAnalyzer::new();
    sema.analyze(&ast).map_err(|e| eprintln!("Semantic: {}", e))?;
    
    let mut codegen = crate::codegen::LLVMGenerator::new();
    let ir = codegen.generate(&ast);
    
    let base_name = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();
    let ll_out = format!("{}.ll", base_name);
    fs::write(&ll_out, ir).map_err(|e| eprintln!("I/O: {}", e))?;
    
    println!("Compiled to LLVM IR -> {}", ll_out);
    println!("Linking native executable '{}'...", base_name);
    
    let status = Command::new("clang")
        .args([&ll_out, "-Wno-override-module", "-o", base_name])
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                println!("Success! Native binary generated. Run it using:");
                println!("  ./{}", base_name);
            } else {
                eprintln!("Error: System C linker 'clang' exited with status code {}.", s);
            }
        }
        Err(e) => {
            eprintln!("Warning: C linker ('clang') could not be executed: {}", e);
            eprintln!("To run your program without compiling, use: krait run {}", path);
        }
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), ()> {
    let source = fs::read_to_string(path).map_err(|e| eprintln!("I/O Error: {}", e))?;
    let tokens = Lexer::tokenize(&source).map_err(|e| eprintln!("Syntax: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().map_err(|e| eprintln!("Parse: {}", e))?;
    
    let mut sema = SemanticAnalyzer::new();
    sema.analyze(&ast).map_err(|e| eprintln!("Semantic: {}", e))?;
    
    let mut interpreter = crate::interpreter::Interpreter::new();
    interpreter.interpret(&ast).map_err(|e| eprintln!("Runtime Error: {}", e))?;
    Ok(())
}

fn check_file(path: &str) -> Result<(), ()> {
    let source = fs::read_to_string(path).map_err(|e| eprintln!("I/O Error: {}", e))?;
    let tokens = Lexer::tokenize(&source).map_err(|e| eprintln!("Syntax: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().map_err(|e| eprintln!("Parse: {}", e))?;
    
    let mut sema = SemanticAnalyzer::new();
    sema.analyze(&ast).map_err(|e| eprintln!("Semantic: {}", e))?;
    println!("All syntax and semantic checks passed.");
    Ok(())
}

fn format_file(path: &str) -> Result<(), ()> {
    let source = fs::read_to_string(path).map_err(|e| eprintln!("I/O Error: {}", e))?;
    let tokens = Lexer::tokenize(&source).map_err(|e| eprintln!("Syntax: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().map_err(|e| eprintln!("Parse: {}", e))?;
    
    let mut formatter = crate::formatter::Formatter::new();
    let formatted = formatter.format(&ast);
    fs::write(path, formatted).map_err(|e| eprintln!("I/O Error: {}", e))?;
    println!("Reformatted file: {}", path);
    Ok(())
}