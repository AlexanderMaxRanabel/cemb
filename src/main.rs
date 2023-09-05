mod printline;

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
                        if let Some(&comparable_one) = tokens.get(1) {
                            if let Some(&comparable_two) = tokens.get(3) {
                                let comparable_one_address: usize = comparable_one.parse().expect("Failed to convert to usize");
                                let comparable_two_address: usize = comparable_two.parse().expect("Failed to convert to usize");

                                let comparable_one_metadata: Vec<String> = (stack[comparable_one_address].clone()).split_whitespace().map(|s: &str| s.to_string()).collect();
                                let comparable_two_metadata: Vec<String> = (stack[comparable_two_address].clone()).split_whitespace().map(|s: &str| s.to_string()).collect();

                                let comparable_one_type: String = comparable_one_metadata[1].clone();
                                let comparable_two_type: String = comparable_two_metadata[1].clone();

                                if comparable_one_type == comparable_two_type {
                                    /*
                                    if[0] 0[1] ==[2] 1[3] printline cemb.stack
                                    */
                                    if let Some(&operator) = tokens.get(2) {
                                        let comparable_one_value: String = comparable_one_metadata[2].clone();
                                        let comparable_two_value: String = comparable_one_metadata[2].clone();
                                        match operator {
                                            "==" => {
                                                if comparable_one_value == comparable_two_value {
                                                    let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                    let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                    println!("{:?}", confirmed_executable_code_vector);
                                                    println!("{:?}", confirmed_executable_code_rest);
                                                    match confirmed_executable_code_vector[0] {
                                                        "printline" => {
                                                            printline::printline(confirmed_executable_code_rest, stack.clone());
                                                        }

                                                        "dealloc_full_stack" => {
                                                            stack.clear();
                                                            stack.shrink_to_fit();
                                                        }

                                                        "dealloc_certain_element" => {
                                                            if let Some(&element_to_remove) = confirmed_executable_code_rest.get(1) {
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
                                                            println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                } else {
                                                    continue
                                                }
                                            }
                                            "!=" => {
                                                if comparable_one_value != comparable_two_value {
                                                    let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                    let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                    match confirmed_executable_code_vector[0] {
                                                        "printline" => {
                                                            printline::printline(confirmed_executable_code_rest, stack.clone());
                                                        }

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
                                                            println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                } else {
                                                    continue
                                                }
                                            }
                                            "<" => {
                                                if comparable_one_type == "String" || comparable_two_type == "String" {
                                                    println!("{}: Strings cannot be compared: {} {}", "Error".red(), comparable_one_type.magenta(), comparable_two_type.magenta());
                                                    std::process::exit(1);
                                                } else if comparable_one_type == "Int" && comparable_two_type == "Int" {
                                                    if let Ok(comparable_one_value_number) = comparable_one_value.parse::<i64>() {

                                                        if let Ok(comparable_two_value_number) = comparable_two_value.parse::<i64>() {

                                                            if comparable_one_value_number < comparable_two_value_number {

                                                                let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                                let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                                match confirmed_executable_code_vector[0] {
                                                                    "printline" => {
                                                                        printline::printline(confirmed_executable_code_rest, stack.clone());
                                                                    }

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
                                                                        println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                                        std::process::exit(1);
                                                                    }
                                                                }
                                                            } else {
                                                                continue
                                                            }
                                                        }
                                                    }
                                                } else if comparable_one_type == "Float" && comparable_two_type == "Float" {
                                                    if let Ok(comparable_one_value_number) = comparable_one_value.parse::<f64>() {

                                                        if let Ok(comparable_two_value_number) = comparable_two_value.parse::<f64>() {

                                                            if comparable_one_value_number < comparable_two_value_number {

                                                                let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                                let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                                match confirmed_executable_code_vector[0] {
                                                                    "printline" => {
                                                                        printline::printline(confirmed_executable_code_rest, stack.clone());
                                                                    }

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
                                                                        println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                                        std::process::exit(1);
                                                                    }
                                                                }
                                                            } else {
                                                                continue
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            ">" => {
                                                if comparable_one_type == "String" || comparable_two_type == "String" {
                                                    println!("{}: Strings cannot be compared: {} {}", "Error".red(), comparable_one_type.magenta(), comparable_two_type.magenta());
                                                    std::process::exit(1);
                                                } else if comparable_one_type == "Int" && comparable_two_type == "Int" {
                                                    if let Ok(comparable_one_value_number) = comparable_one_value.parse::<i64>() {

                                                        if let Ok(comparable_two_value_number) = comparable_two_value.parse::<i64>() {

                                                            if comparable_one_value_number > comparable_two_value_number {

                                                                let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                                let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                                match confirmed_executable_code_vector[0] {
                                                                    "printline" => {
                                                                        printline::printline(confirmed_executable_code_rest, stack.clone());
                                                                    }

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
                                                                        println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                                        std::process::exit(1);
                                                                    }
                                                                }
                                                            } else {
                                                                continue
                                                            }
                                                        }
                                                    }
                                                } else if comparable_one_type == "Float" && comparable_two_type == "Float" {
                                                    if let Ok(comparable_one_value_number) = comparable_one_value.parse::<f64>() {

                                                        if let Ok(comparable_two_value_number) = comparable_two_value.parse::<f64>() {

                                                            if comparable_one_value_number > comparable_two_value_number {

                                                                let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                                let confirmed_executable_code_rest: Vec<&str> = tokens[4..].to_vec();
                                                                match confirmed_executable_code_vector[0] {
                                                                    "printline" => {
                                                                        printline::printline(confirmed_executable_code_rest, stack.clone());
                                                                    }

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
                                                                        println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                                        std::process::exit(1);
                                                                    }
                                                                }
                                                            } else {
                                                                continue
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            &_ => {
                                                println!("{}: Unknown Operator:  {}", "Error".red(), operator.magenta());
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                } else {
                                    println!("{}: Type Mismatch: {} {}", "Error".red(), comparable_one_type.magenta(), comparable_two_type.magenta());
                                    break
                                }
                            }
                        }
                    }

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
