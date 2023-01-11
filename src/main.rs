#![feature(let_chains)]

use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use clap::{arg, command};
use colored::Colorize;
use crate::parser::parse;
use crate::tokenizer::tokenizer;

mod tokenizer;
mod parser;
mod transformer;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            command!("compile")
                .arg(arg!(-s --source <FILE> "the source file path"))
                .arg(arg!(-o --output <FILE> "the output file path"))
        )
        .subcommand(
            command!("exec")
                .arg(arg!(<FILE> "the source bytecode file path"))
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("compile") {
        // 编译相关逻辑
        if let Some(source_path) = matches.get_one::<String>("source")
            && let Some(output_path) = matches.get_one::<String>("output") {
            let mut buf = String::new();
            File::open(source_path)?.read_to_string(&mut buf)?;
            let tokens = tokenizer(&buf);
            println!("{:?}", tokens);
            let ast = parse(tokens).unwrap();

        }
    } else if let Some(matches) = matches.subcommand_matches("exec")  {
        // 执行相关逻辑
        if let Some(file_path) = matches.get_one::<String>("FILE") {
            exec(file_path)?;
        } else {
            error("no input files")
        }
    }
    Ok(())
}

fn exec(file_path: &str) -> Result<()> {
    let file = File::open(file_path);
    if let Ok(mut file) = file {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let mut stack: Vec<i64> = vec![];
        buf.split("\n")
            .for_each(|s: &str| {
                match s {
                    s if s.starts_with("push") => {
                        let spilt: Vec<&str> = s.split_whitespace().collect();
                        let num = spilt[1].parse::<i64>().unwrap_or_run(|| error("parse failed"));
                        stack.push(num);
                    }
                    s if s.starts_with("add") => {
                        let num2 = stack.pop().unwrap();
                        let num1 = stack.pop().unwrap();
                        stack.push(num1 + num2)
                    }
                    s if s.starts_with("sub") => {
                        let num2 = stack.pop().unwrap();
                        let num1 = stack.pop().unwrap();
                        stack.push(num1 - num2)
                    }
                    s if s.starts_with("mul") => {
                        let num2 = stack.pop().unwrap();
                        let num1 = stack.pop().unwrap();
                        stack.push(num1 * num2)
                    }
                    s if s.starts_with("div") => {
                        let num2 = stack.pop().unwrap();
                        let num1 = stack.pop().unwrap();
                        stack.push(num1 / num2)
                    }
                    s if s.starts_with("ret") => {
                        println!("{}", stack.pop().unwrap());
                        exit(0)
                    }
                    &_ => { error("parse failed") }
                };
            });
            error("the program doesn't return any value")
    } else {
        error("file not found or cannot open file")
    }
    Ok(())
}

fn error(reason: &str) {
    println!("{} {}", "fatal error:".bold().bright_red(), reason.bright_white());
    exit(1)
}

trait UnwrapOrRun<T> {
    fn unwrap_or_run<F>(self, func: F) -> T
        where F: Fn();
}

impl <T, E: Debug> UnwrapOrRun<T> for std::result::Result<T, E> {
    fn unwrap_or_run<F>(self, func: F) -> T where F: Fn() {
        if let Ok(value) = self {
            value
        } else {
            func();
            exit(1)
        }
    }
}