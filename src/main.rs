use anyhow::{Context, Result};
use std::{
    cmp, env, fs,
    io::{self, BufRead, Write},
    process,
};

use ast::AstPrinter;
use parser::Parser;
use scanner::Scanner;

mod ast;
mod parser;
mod scanner;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    match args.len().cmp(&2) {
        cmp::Ordering::Greater => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
        cmp::Ordering::Equal => {
            run_file(&args[1])?;
        }
        _ => {
            run_prompt()?;
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let s = fs::read_to_string(path).context("couldn't read input file")?;
    run(&s)
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let expr = parser.parse()?;
    println!("{}", AstPrinter.print(&expr));
    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        line.clear();

        print!("> ");
        stdout.flush()?;

        stdin.lock().read_line(&mut line)?;
        if let Err(e) = run(&line) {
            eprint!("{}", e);
        };
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, at: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, at, message);
}
