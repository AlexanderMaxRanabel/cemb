use std::env;

use std::fs::File;
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
                                                    if let Ok(_number) = value.parse::<i32>() {
                                                        let metadata: String = format!("{} {} {}", var_name, var_type, value);
                                                        stack.push(metadata);
                                                        println!("{:?}", stack);
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
                                                        println!("{:?}", stack)
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
                        if let Some(&second_token) = tokens.get(1) {
                            match second_token {
                                "cemb.stack" => {
                                    if stack.len() > 0 {
                                        println!("{:?}", stack);
                                    } else {
                                        println!("[]")
                                    }
                                },

                                "cemb.fromstack" => {
                                    if let Some(&stack_element) = tokens.get(2) {
                                        let stack_element: usize = stack_element.parse().expect("Failed to convert to Usize");
                                        let certain_element = stack[stack_element].clone();
                                        println!("{}", certain_element);
                                    }
                                },

                                _ => {
                                    if let Some(&last_element) = tokens.last() {
                                        if second_token.starts_with("'") && last_element.ends_with("'") {
                                            let combined_str: String = tokens[1..].join(" ");
                                            println!("{}", combined_str);
                                        } else {
                                            println!("Usage")
                                        }
                                    }
                                },
                            }
                        }
                    },

                    "dealloc_full_stack" => {
                        stack.clear();
                        stack.shrink_to_fit();
                    }

                    "dealloc_certain_element" => {
                        if let Some(&element_to_remove) = tokens.get(1) {
                            let element_to_remove: usize = element_to_remove.parse().expect("Failed to convert to usize");
                            if (element_to_remove) < stack.len() {
                                stack.remove(element_to_remove);
                            } else {
                                println!("{} {}", " Error: Cannot remove element because it does not exist".red(), element_to_remove.to_string());
                                std::process::exit(0);
                            }
                        }
                    }
                    _ => {
                        println!("{} {}", "Error: Unknown Keyword: ".red(), tokens[0]);
                        std::process::exit(0)
                    },
                }
            }
        }
    } else {
        println!("No file provided");
    }
    Ok(())
}
