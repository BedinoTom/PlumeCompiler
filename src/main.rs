extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::{cmp::Ordering, fmt::Binary};
use std::ops::{Add,Sub};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;


struct JumpTable{
    jump_table : Vec<Jump>
}

impl JumpTable {
    fn find_position_by_jump_label(&self,label:String) -> Option<&i64> {
        for jump in self.jump_table.iter(){
            if jump.label == label {
                return Some(&jump.line);
            }
        }

        None
    }
}

struct Jump {
    label : String,
    line : i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Operande {
    phase : String,
    r#type : String,
    size : usize,
    offset : i8,
    offset_bin : i8,
    optional : bool
}

#[derive(Serialize, Deserialize)]
struct Instruct {
    name : String,
    subname : String,
    opcode : String,
    familly : String,
    signature : String,
    operandes: Vec<Operande>,
}

impl Instruct {
    fn find_operande_by_offset(&self, pos :i8) -> Option<&Operande> {
        for operande in &self.operandes{
            if operande.offset == pos{
                return Some(operande);
            }
        }
        return None;
    }
}

#[derive(Serialize, Deserialize)]
struct Instructs {
    opcodes: Vec<Instruct>
}

impl Instructs {
    fn match_instruction(&self, query:&InstructSet) -> Option<&Instruct> {
        for instruct in self.opcodes.iter(){
            
            if format!("{}{}", instruct.name,instruct.subname.to_ascii_uppercase()) == query.name {
                if instruct.signature.len()>query.signature.len() {
                    if instruct.signature.contains('w'){
                        let new_signature = format!("w{}", query.signature);
                        if instruct.signature==new_signature{
                            return Some(instruct);
                        }
                    }
                }else{
                    if instruct.signature==query.signature{
                        return Some(instruct);
                    }
                }
            }
        }
        return None;
    }
}

struct InstructSet {
    name : String,
    signature : String,
    operandes : Vec<OperandeValue>,
    line : i64
}

struct OperandeValue{
    operande : Operande,
    value_int : i32,
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

fn parse_instructs(data : String) -> Result<Instructs> {
    let p: Result<Instructs> = serde_json::from_str(data.as_str());
    p
}

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$x}", n, len)
}

fn write_result(bin : Vec<String>, filename : String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let word = bin.join(" ");
    file.write_all(b"v2.0 raw\n")?;
    file.write_all(word.as_bytes())?;
    Ok(())
}

fn convert_i8_to_usize(v:i8) -> Option<usize> {
    if v < 0 || v > std::i8::MAX {
        None
    }else {
        Some(v as usize)
    }
}

fn convert_numeric_to_binary<T: Add<Output=T> + Sub<Output=T> + Ord + Binary>(n: T, size:usize) -> String{
    let bin = format!("{:01$b}", n, size);

    bin[bin.len()-size..bin.len()].to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        process::exit(-1);
    }

    let file_path = "opcodes_3.json";
    println!("Load instructs file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let instructs = parse_instructs(contents.clone());

    let content_instructs=match instructs {
        Ok(o) => {
            println!("There is {} instructions", o.opcodes.len());
            o
        },
        Err(e) => {
            println!("{}",e);
            println!("Abort !");
            process::exit(-1);
        }
    };

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
    let mut lines_count : i64 = 0;
    let mut count_label : i64 = 0;

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

    let jump_table_obj = JumpTable{
        jump_table : jump_table
    };

    let mut instructions_vector : Vec<InstructSet> = Vec::new();
    let mut line_count : i64 = 0;
    for line in lines_iter.into_inner() {
        match line.as_rule() {
            Rule::instruct_line => {
                println!("Instruction {}",line.as_str());
                for instruct_iter in line.into_inner(){
                    match instruct_iter.as_rule() {
                        Rule::instruct => {
                            let mut inner_instructs = instruct_iter.into_inner();
                            let opcode_string = inner_instructs.next().unwrap().as_str();
                            let mut operande_vector : Vec<OperandeValue> = Vec::new();
                            let mut signature_vector : Vec<char> = Vec::new();
                            while let Some(operande) = inner_instructs.next() {
                                let mut inner_operande = operande.into_inner();
                                let operande_word = inner_operande.next().unwrap();
                                match operande_word.as_rule() {
                                    Rule::term => {
                                        let mut inner_term = operande_word.into_inner();
                                        let term = inner_term.next().unwrap();
                                        match term.as_rule() {
                                            Rule::register => {
                                                let value : i32 = match term.into_inner().next() {
                                                    Some(v) => {
                                                        match v.as_str().parse() {
                                                            Ok(v) => {v},
                                                            Err(e) => {
                                                                println!("value parse error {}",e);
                                                                process::exit(-1);
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        0
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
                                                signature_vector.push('r');
                                            },
                                            Rule::immediate => {
                                                let value : i32 = match term.into_inner().next().unwrap().as_str().parse() {
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
                                                signature_vector.push('i');
                                            },
                                            _ => {}
                                        }
                                    },
                                    Rule::expression => {
                                        let mut inner_expression = operande_word.into_inner().into_iter();
                                        inner_expression.next();
                                        let value : i32 = match inner_expression.next() {
                                            Some(v) => {
                                                let string_value = match v.into_inner().next(){
                                                    Some(o) => {
                                                        match o.into_inner().next() {
                                                            Some(n) => {
                                                                match n.into_inner().next(){
                                                                    Some(i) => {
                                                                        i
                                                                    },
                                                                    None => {
                                                                        println!("value parse error");
                                                                        process::exit(-1);
                                                                    }
                                                                }
                                                            },
                                                            None => {
                                                                println!("value parse error ");
                                                                process::exit(-1);
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        println!("value parse error ");
                                                        process::exit(-1);
                                                    }
                                                };
                                                match string_value.as_str().parse() {
                                                    Ok(v) => {v},
                                                    Err(e) => {
                                                        println!("value parse error {}",e);
                                                        process::exit(-1);
                                                    }
                                                }
                                            
                                            },
                                            None => {
                                                0
                                            }
                                        };
                                        operande_vector.push(OperandeValue { 
                                            operande: Operande{
                                                phase : "input".to_string(),
                                                r#type : "term".to_string(),
                                                size : 1,
                                                offset : 0,
                                                offset_bin : 0,
                                                optional : false
                                            }, 
                                            value_int: value, 
                                            value_string: "".to_string()
                                        });
                                        signature_vector.push('t');
                                    },
                                    Rule::label_name => {
                                        let label_name = operande_word.into_inner().into_iter().next().unwrap();
                                        println!("Label {}", label_name.as_str());
                                        operande_vector.push(OperandeValue { 
                                            operande: Operande{
                                                phase : "input".to_string(),
                                                r#type : "label".to_string(),
                                                size : 1,
                                                offset : 0,
                                                offset_bin : 0,
                                                optional : false
                                            }, 
                                            value_int: 0, 
                                            value_string: label_name.as_str().to_string()
                                        });
                                        signature_vector.push('l');
                                    }
                                    _ => {}
                                }
                            }

                            instructions_vector.push(InstructSet { 
                                name: opcode_string.to_string(), 
                                signature: signature_vector.into_iter().collect(), 
                                operandes: operande_vector,
                                line : line_count
                            });
                        },
                        _ => {}
                    }
                }
                line_count = line_count +1;
            }
            _ => {}
        }
    }

    let mut word_bin : Vec<String> = Vec::new();
    for instruct in instructions_vector.iter() {
        let match_instruct = match content_instructs.match_instruction(instruct){
            Some(i) => i,
            None => {
                println!("Mismatch Instruct {} {}", instruct.name, instruct.signature);
                process::exit(-1);
            }
        };
        let mut instruct_bin : Vec<String> = Vec::with_capacity(instruct.operandes.len()+1);
        instruct_bin.resize(instruct.operandes.len()+1,"".to_string());
        let operate_iter = instruct.operandes.iter();
        let mut count_operande = 0;
        for operande_value in operate_iter{
            let match_operande = match match_instruct.find_operande_by_offset(count_operande){
                Some(v)=> v,
                None => {
                    println!("Error Parse Instruct {} {} {}", instruct.name, instruct.signature, count_operande);
                    process::exit(-1);
                }
            };

            if operande_value.operande.r#type == "register" || operande_value.operande.r#type == "imm" || operande_value.operande.r#type == "term"{
                //instruct_bin[match_operande.offset_bin] = format!("{:01$b}", operande_value.value_int, match_operande.size);
                match convert_i8_to_usize(match_operande.offset_bin){
                    Some(i) => {
                        instruct_bin[i] = convert_numeric_to_binary(operande_value.value_int, match_operande.size);//format!("{:01$b}", operande_value.value_int, match_operande.size);
                        println!("bin {}", instruct_bin[i]);
                    },
                    None => {}
                }
            }else if operande_value.operande.r#type=="label" {
                match convert_i8_to_usize(match_operande.offset_bin){
                    Some(i) => {
                        match jump_table_obj.find_position_by_jump_label(operande_value.value_string.clone()) {
                            Some(v) => {
                                let encoded_jump = v - instruct.line - 3;
                                println!("Line {}" , encoded_jump);
                                instruct_bin[i] = convert_numeric_to_binary(encoded_jump, match_operande.size);
                            },
                            None => {
                                println!("Error Unknown label {} {} {}", instruct.name, operande_value.value_string, count_operande);
                                process::exit(-1); 
                            }
                        }
                    },
                    None => {}
                }
            }

            count_operande += 1;
        }
        instruct_bin.insert(0,match_instruct.opcode.clone());
        println!("Str : {}" , instruct_bin.join("").as_str());
        word_bin.push(to_hex(&instruct_bin.join("").as_str(),4));

        
    }

    println!("Bin Table:");
        for bin in word_bin.iter(){
            println!("{}", bin);
        }
}
