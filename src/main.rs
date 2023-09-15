mod printline;
mod test_debug;

use std::env;

use std::fs::{File};
use std::io::{BufReader, BufRead};
use colored::*;

use test_debug::*;

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
                                        let comparable_two_value: String = comparable_two_metadata[2].clone();
                                        match operator {
                                            "==" => {
                                                let confirmed_executable_code_vector: Vec<&str> = tokens[4..].to_vec();
                                                let else_index = confirmed_executable_code_vector.iter().position(|&c| c == "else");
                                                let mut confirmed_executable_code_rest: Vec<&str> = Vec::new();

                                                match else_index {
                                                    Some(index) => {
                                                        confirmed_executable_code_rest = confirmed_executable_code_vector[..=index].to_vec();
                                                        confirmed_executable_code_rest.remove(0);
                                                        confirmed_executable_code_rest.pop();
                                                    },
                                                    None => {
                                                        println!("{}: No Else is found in if statement", "Error".red());
                                                    },
                                                }

                                                let else_executable_code_vector: Vec<_> = confirmed_executable_code_vector.iter()
                                                    .skip_while(|&&x| x != "else")
                                                    .cloned()
                                                    .collect();

                                                let else_executable_code_rest: Vec<&str> = else_executable_code_vector[2..].to_vec();
                                                /*
                                                _test_debug_vec_mc(confirmed_executable_code_vector.clone());
                                                _test_debug_string("INDICATOR 1".to_string());
                                                _test_debug_vec_mc(else_executable_code_vector.clone());
                                                _test_debug_string("INDICATOR 2".to_string());
                                                _test_debug_vec_mc(confirmed_executable_code_rest.clone());
                                                _test_debug_string("INDICATOR 3".to_string());
                                                _test_debug_vec_mc(else_executable_code_rest.clone());
                                                _test_debug_string("INDICATOR 4".to_string());
                                                _test_debug_string(else_index.clone().expect("Failed to clone").to_string()); */

                                                if comparable_one_value == comparable_two_value {
                                                    match confirmed_executable_code_vector[0] {
                                                        "printline" => {
                                                            printline::printline(confirmed_executable_code_rest, stack.clone());
                                                        },

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
                                                    match else_executable_code_vector[0] {
                                                        "printline" => {
                                                            printline::printline(else_executable_code_rest, stack.clone());
                                                        },

                                                        "dealloc_full_stack" => {
                                                            stack.clear();
                                                            stack.shrink_to_fit();
                                                        },

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
                                                        },

                                                        _ => {
                                                            println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_code_vector[0].magenta());
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                }
                                            },

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
                                            },

                                            "<" => {
                                                match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                                                    ("Int", "Int") => {
                                                        let comparable_one_value_integer: i64 = comparable_one_value.parse().expect("Failed at conversion of value to Signed Integer 64");
                                                        let comparable_two_value_integer: i64 = comparable_two_value.parse().expect("Failed at conversion of value to Signed Integer 64");

                                                        if comparable_one_value_integer < comparable_two_value_integer {
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
                                                        }
                                                    },

                                                    ("Float", "Float") => {
                                                        let comparable_one_value_float: f64 = comparable_one_value.parse().expect("Failed to convert to Signed Floating Point 64");
                                                        let comparable_two_value_float: f64 = comparable_two_value.parse().expect("Failed to convert to Signed Floating Point 64");

                                                        if comparable_one_value_float < comparable_two_value_float {
                                                            if comparable_one_value_float < comparable_two_value_float {
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
                                                            }
                                                        }
                                                    },

                                                    _ => {
                                                        println!("{}: Strings or any types cannot be compared: {} {}", "Error".red(), comparable_one_type.magenta(), comparable_two_type.magenta());
                                                        std::process::exit(1);
                                                    }
                                                }
                                            },

                                            ">" => {
                                                match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                                                    ("Int", "Int") => {
                                                        let comparable_one_value_integer: i64 = comparable_one_value.parse().expect("Failed at conversion of value to Signed Integer 64");
                                                        let comparable_two_value_integer: i64 = comparable_two_value.parse().expect("Failed at conversion of value to Signed Integer 64");

                                                        if comparable_one_value_integer > comparable_two_value_integer {
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
                                                        }
                                                    },

                                                    ("Float", "Float") => {
                                                        let comparable_one_value_float: f64 = comparable_one_value.parse().expect("Failed to convert to Signed Floating Point 64");
                                                        let comparable_two_value_float: f64 = comparable_two_value.parse().expect("Failed to convert to Signed Floating Point 64");

                                                        if comparable_one_value_float > comparable_two_value_float {
                                                            if comparable_one_value_float < comparable_two_value_float {
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
                                                            }
                                                        }
                                                    },

                                                    _ => {
                                                        println!("{}: Strings or any types cannot be compared: {} {}", "Error".red(), comparable_one_type.magenta(), comparable_two_type.magenta());
                                                        std::process::exit(1);
                                                    }
                                                }
                                            },

                                            _ => {
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
