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

pub struct Jump {
    label : String,
    line : u64,
}

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

    let mut jump_table : Vec<Jump> = Vec::new();
    let mut lines_count : u64 = 0;

    let iter_label = lines_iter.clone();
    for line in iter_label.into_inner() {
        match line.as_rule() {
            Rule::label_line => {
                let mut label_rules = line.into_inner();
                let mut string_label_rules = label_rules.next().unwrap().into_inner();
                let label_str = string_label_rules.next().unwrap().as_str().to_string();
                jump_table.push(Jump { label: label_str, line: lines_count });
            }
            _ => {
            }
        }
        lines_count+=1;
    }


    for line in lines_iter.into_inner() {
        match line.as_rule() {
            Rule::instruct_line => {
                println!("Instruction {}",line.as_str());
            }
            _ => {
            }
        }
    }
}
