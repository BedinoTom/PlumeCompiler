extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::fs;
use std::env;
use std::process;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

pub struct Jump {
    label : String,
    line : u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Operande {
    phase : String,
    r#type : String,
    size : usize,
    offset : usize,
    offset_bin : usize,
    optional : bool
}

#[derive(Serialize, Deserialize)]
struct Instruct {
    name : String,
    subname : String,
    regex_matcher : String,
    opcode : String,
    familly : String,
    signature : String,
    operandes: Vec<Operande>,
}

#[derive(Serialize, Deserialize)]
struct Instructs {
    opcodes: Vec<Instruct>
}

struct InstructSet {
    name : String,
    signature : String,
    operandes : Vec<OperandeValue>
}

struct OperandeValue{
    operande : Operande,
    value_int : u64,
    value_string : String
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
    let mut count_label : u64 = 0;

    let iter_label = lines_iter.clone();
    for line in iter_label.into_inner() {
        match line.as_rule() {
            Rule::label_line => {
                let mut label_rules = line.into_inner();
                let mut string_label_rules = label_rules.next().unwrap().into_inner();
                let label_str = string_label_rules.next().unwrap().as_str().to_string();
                
                count_label+=1;
                jump_table.push(Jump { label: label_str, line: (lines_count+1)-count_label });
                lines_count+=1;
            },
            Rule::instruct_line => {
                lines_count+=1;
            },
            _ => {
            }
        }
    }

    for jump in jump_table.iter(){
        println!("Jump");
        println!("{}",jump.label);
        println!("{}",jump.line);
    }

    let mut instructions_vector : Vec<InstructSet> = Vec::new();
    for line in lines_iter.into_inner() {
        match line.as_rule() {
            Rule::instruct_line => {
                println!("Instruction {}",line.as_str());
                for instruct_iter in line.into_inner(){
                    match instruct_iter.as_rule() {
                        Rule::instruct => {
                            let mut inner_instructs = instruct_iter.into_inner();
                            println!("Opcode {}", inner_instructs.next().unwrap().as_str());
                            let mut operande_vector : Vec<OperandeValue> = Vec::new();
                            while let Some(operande) = inner_instructs.next() {
                                let mut inner_operande = operande.into_inner();
                                let operande_word = inner_operande.next().unwrap();
                                match operande_word.as_rule() {
                                    Rule::term => {
                                        let mut inner_term = operande_word.into_inner();
                                        let term = inner_term.next().unwrap();
                                        match term.as_rule() {
                                            Rule::register => {
                                                let value : u64 = match term.into_inner().next().unwrap().as_str().parse() {
                                                    Ok(v) => {v},
                                                    Err(e) => {
                                                        println!("value parse error {}",e);
                                                        process::exit(-1);
                                                    }
                                                };
                                                operande_vector.push(OperandeValue { 
                                                        operande: Operande{
                                                            phase : "input".to_string(),
                                                            r#type : "register".to_string(),
                                                            size : 1,
                                                            offset : 0,
                                                            offset_bin : 0,
                                                            optional : false
                                                        }, 
                                                        value_int: value, 
                                                        value_string: "".to_string() 
                                                    });
                                            },
                                            Rule::immediate => {
                                                let value : u64 = match term.into_inner().next().unwrap().as_str().parse() {
                                                    Ok(v) => {v},
                                                    Err(e) => {
                                                        println!("value parse error {}",e);
                                                        process::exit(-1);
                                                    }
                                                };
                                                operande_vector.push(OperandeValue { 
                                                        operande: Operande{
                                                            phase : "input".to_string(),
                                                            r#type : "imm".to_string(),
                                                            size : 1,
                                                            offset : 0,
                                                            offset_bin : 0,
                                                            optional : false
                                                        }, 
                                                        value_int: value, 
                                                        value_string: "".to_string() 
                                                    });
                                            },
                                            _ => {}
                                        }
                                    },
                                    Rule::expression => {
                                        let mut inner_expression = operande_word.into_inner().into_iter();
                                        //inner_expression.next();
                                        let offset_expression = inner_expression.next().unwrap();
                                        println!("Expression {}", offset_expression.as_str());
                                    },
                                    Rule::label_call => {
                                        let label_name = operande_word.into_inner().into_iter().next().unwrap();
                                        println!("Label {}", label_name.as_str());
                                        operande_vector.push(OperandeValue { 
                                            operande: Operande{
                                                phase : "input".to_string(),
                                                r#type : "imm".to_string(),
                                                size : 1,
                                                offset : 0,
                                                offset_bin : 0,
                                                optional : false
                                            }, 
                                            value_int: 0, 
                                            value_string: label_name.as_str().to_string()
                                        });
                                    }
                                    _ => {
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
            _ => {
            }
        }
    }
}
