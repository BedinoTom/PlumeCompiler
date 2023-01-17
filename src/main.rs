extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::fs;
use std::env;
use std::process;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

fn load_file(file_path:String) -> Option<String> {
    println!("{}",file_path.clone());
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(c) => {
            Some(c)
        },
        Err(e) => {
            println!("Error while reading file : {}",e);
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        process::exit(-1);
    }

    println!("Load source file {}", args[1].clone());
    let content = match load_file(args[1].clone()) {
        Some(c) => {
            c
        },
        None => {
            process::exit(-1);
        }
    };

    let mut lines = GrammarParser::parse(Rule::assemblyfile, content.as_str()).unwrap_or_else(|e| panic!("{}", e));
    let lines_iter = match lines.next(){
        Some(iter) => iter,
        None => {
            process::exit(-1);
        }
    };

    let iter_label = lines_iter.clone();
    for line in iter_label.into_inner() {
        match line.as_rule() {
            Rule::label_line => {
                println!("{}",line.as_str());
            }
            _ => {
            }
        }
    }


    for line in lines_iter.into_inner() {
        match line.as_rule() {
            Rule::instruct_line => {
                println!("{}",line.as_str());
            }
            _ => {
            }
        }
    }
}
