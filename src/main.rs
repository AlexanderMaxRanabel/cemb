mod printline;
mod external;
mod memory_management;
mod if_expr;
mod forever;

use std::env;

use std::fs::{File};
use std::io::{BufReader, BufRead};

use colored::*;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file = args.get(1).cloned().unwrap_or_else(|| {
            println!("No file provided");
            std::process::exit(1);
        });

        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let mut stack: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if let Some(&first_token) = tokens.get(0) {
                match first_token {
                    "let" => {
                        //let0 x1 ::2 int3 =4 45
                        if let Some(&var_name) = tokens.get(1) {
                            if let Some(&type_indicator) = tokens.get(2) {
                                if type_indicator == "::" {
                                    if let Some(&var_type) = tokens.get(3) {
                                        match var_type {
                                            "String" => {
                                                if let Some(&string_first_part) = tokens.get(5) {
                                                    if let Some(&string_last_part) = tokens.last() {
                                                        if string_first_part.starts_with("'") && string_last_part.ends_with("'") {
                                                            let value: String = tokens[5..].join(" ");
                                                            let metadata: String = format!("{} {} {}", var_name, var_type, value);
                                                            stack.push(metadata);
                                                        } else {
                                                            println!("Usage: let str :: String = 'Hello world'");
                                                            std::process::exit(0);
                                                        }
                                                    }
                                                }
                                            },

                                            "Int" => {
                                                if let Some(&value) = tokens.get(5) {
                                                    if let Ok(_number) = value.parse::<i64>() {
                                                        let metadata: String = format!("{} {} {}", var_name, var_type, value);
                                                        stack.push(metadata);
                                                    } else {
                                                        println!("{} {}", "Not a piece of valid integer: ".red(), value);
                                                    }
                                                }
                                            },

                                            "Float" => {
                                                if let Some(&value) = tokens.get(5) {
                                                    if let Ok(_number) = value.parse::<f64>() {
                                                        let metadata: String = format!("{} {} {}", var_name, var_type, value);
                                                        stack.push(metadata);
                                                    } else {
                                                        println!("{} {}", "Not a piece of float: ".red(), value);
                                                    }
                                                }
                                            },

                                            "Char" => {
                                                if let Some(&value) = tokens.get(5) {
                                                    if value.len() == 1 {
                                                        let metadata: String = format!("{} {} {}", var_name, var_type, value);
                                                        stack.push(metadata);
                                                    } else {
                                                        println!("{}: {}", "Not a valid piece of char due to length".red(), value);
                                                    }
                                                }
                                            },

                                            _ => {
                                                println!("{} {}", "Error: Unknown Type: ".red(), var_type);
                                                std::process::exit(0)
                                            }
                                        }
                                    }
                                } else {
                                    println!("Usage: let x :: TYPE = VALUE");
                                }
                            }
                        }
                    },
                    
                    "printline" => {
                        printline::printline(tokens, stack.clone());
                    },

                    "if" => {
                        stack = if_expr::if_expr(tokens.clone(), stack.clone()); 
                    },

                    "external" => {
                        let file_stdout = external::external(tokens);
                        println!("{}", file_stdout);
                    },

                    "forever" => {
                        stack = forever::forever(tokens.clone(), stack.clone());
                    },

                    "dealloc_full_stack" => {
                        stack = memory_management::dealloc_full_stack(stack);
                    },

                    "dealloc_certain_element" => {
                        stack = memory_management::dealloc_certain_element(stack, tokens.clone());
                    },

                    "//" => {
                        continue;
                    },

                    _ => {
                        println!("{}: Unknown Keyword: {}", "Error".red(), tokens[0]);
                        std::process::exit(0)
                    },
                }
            }
        }
    } else {
        println!("{}: No file provided", "Error".red());
    }
    Ok(())
}
