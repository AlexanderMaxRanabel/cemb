use crate::{external::*, forever::*, memory_management::*, printline::*, while_loop::*};

use colored::*;

fn executable_runner(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    //Much better than writing the same stuff 7 thousand times
    match tokens[0] {
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
                                            if string_first_part.starts_with("'")
                                                && string_last_part.ends_with("'")
                                            {
                                                let value: String = tokens[5..].join(" ");
                                                let metadata: String =
                                                    format!("{} {} {}", var_name, var_type, value);
                                                stack.push(metadata);
                                            } else {
                                                println!(
                                                    "Usage: let str :: String = 'Hello world'"
                                                );
                                                std::process::exit(0);
                                            }
                                        }
                                    }
                                }

                                "Int" => {
                                    if let Some(&value) = tokens.get(5) {
                                        if let Ok(_number) = value.parse::<i64>() {
                                            let metadata: String =
                                                format!("{} {} {}", var_name, var_type, value);
                                            stack.push(metadata);
                                        } else {
                                            println!(
                                                "{} {}",
                                                "Not a piece of valid integer: ".red(),
                                                value
                                            );
                                        }
                                    }
                                }

                                "Float" => {
                                    if let Some(&value) = tokens.get(5) {
                                        if let Ok(_number) = value.parse::<f64>() {
                                            let metadata: String =
                                                format!("{} {} {}", var_name, var_type, value);
                                            stack.push(metadata);
                                        } else {
                                            println!(
                                                "{} {}",
                                                "Not a piece of float: ".red(),
                                                value
                                            );
                                        }
                                    }
                                }

                                "Char" => {
                                    if let Some(&value) = tokens.get(5) {
                                        if value.len() == 1 {
                                            let metadata: String =
                                                format!("{} {} {}", var_name, var_type, value);
                                            stack.push(metadata);
                                        } else {
                                            println!(
                                                "{}: {}",
                                                "Not a valid piece of char due to length".red(),
                                                value
                                            );
                                        }
                                    }
                                }

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
        }

        "printline" => {
            printline(tokens.clone(), stack.clone());
        }

        "dealloc_full_stack" => {
            stack = dealloc_full_stack(stack);
        }

        "dealloc_certain_element" => {
            stack = dealloc_certain_element(stack, tokens);
        }

        "external" => {
            let file_stdout = external(tokens);
            println!("{}", file_stdout);
        }

        "forever" => {
            stack = forever(tokens.clone(), stack.clone());
        }

        "while" => {
            stack = while_loop(tokens.clone(), stack.clone());
        }

        _ => {
            println!(
                "{}: Unknown Keyword: {}",
                "Error".red(),
                tokens[0].magenta()
            );
            std::process::exit(1);
        }
    }
    return stack;
}

pub fn if_expr(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    if let Some(&comparable_one_address_raw) = tokens.get(1) {
        if let Some(&comparable_two_address_raw) = tokens.get(3) {
            if let Some(&operator) = tokens.get(2) {
                let comparable_one_address: usize = comparable_one_address_raw
                    .parse::<usize>()
                    .expect("Failed to convert");
                let comparable_two_address: usize = comparable_two_address_raw
                    .parse::<usize>()
                    .expect("Failed to convert");

                let comparable_one_metadata: Vec<String> = (stack[comparable_one_address].clone())
                    .split_whitespace()
                    .map(|s: &str| s.to_string())
                    .collect();
                let comparable_two_metadata: Vec<String> = (stack[comparable_two_address].clone())
                    .split_whitespace()
                    .map(|s: &str| s.to_string())
                    .collect();

                let comparable_one_type: String = comparable_one_metadata[1].clone();
                let comparable_two_type: String = comparable_two_metadata[1].clone();

                if comparable_one_type != comparable_two_type {
                    println!("{}: Type Mismatch", "Error".red());
                    std::process::exit(1);
                }

                let comparable_one_value: String = comparable_one_metadata[2].clone();
                let comparable_two_value: String = comparable_two_metadata[2].clone();

                let confirmed_executable_tokens_double: Vec<_> = tokens
                    .iter()
                    .skip_while(|&&c| c != "do")
                    .skip(1)
                    .take_while(|&&c| c != "else")
                    .collect();

                let else_executable_tokens_double: Vec<_> = tokens
                    .iter()
                    .skip_while(|&&c| c != "else")
                    .skip(1)
                    .take_while(|&&c| c != "end")
                    .collect();

                let else_executable_tokens: Vec<&str> =
                    else_executable_tokens_double.iter().map(|&&s| s).collect();
                let confirmed_executable_tokens: Vec<&str> = confirmed_executable_tokens_double
                    .iter()
                    .map(|&&s| s)
                    .collect();

                match operator {
                    "==" => {
                        if comparable_one_value == comparable_two_value {
                            stack = executable_runner(confirmed_executable_tokens.clone(), stack);
                        } else {
                            stack = executable_runner(else_executable_tokens.clone(), stack);
                        }
                    }

                    "!=" => {
                        if comparable_one_value != comparable_two_value {
                            stack = executable_runner(confirmed_executable_tokens.clone(), stack);
                        } else {
                            stack = executable_runner(else_executable_tokens.clone(), stack);
                        }
                    }

                    ">" => {
                        let mut comparable_one_value_number: f64 = 0.0;
                        let mut comparable_two_value_number: f64 = 0.0;

                        match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                            ("Float", "Float") | ("Int", "Int") => {
                                comparable_one_value_number = comparable_one_value
                                    .clone()
                                    .parse()
                                    .expect("Failed to parse");
                                comparable_two_value_number = comparable_two_value
                                    .clone()
                                    .parse()
                                    .expect("Failed to parse");
                            }

                            _ => {
                                println!(
                                    "{}: Undesired Types: {} {}. Code: {}",
                                    "Error".red(),
                                    comparable_one_type.cyan(),
                                    comparable_two_type.cyan(),
                                    "HRK-MRKM-1Q84".magenta()
                                );
                            }
                        }

                        if comparable_one_value_number > comparable_two_value_number {
                            stack = executable_runner(confirmed_executable_tokens.clone(), stack);
                        } else {
                            stack = executable_runner(else_executable_tokens.clone(), stack);
                        }
                    }

                    "<" => {
                        let mut comparable_one_value_number: f64 = 0.0;
                        let mut comparable_two_value_number: f64 = 0.0;

                        match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                            ("Float", "Float") | ("Int", "Int") => {
                                comparable_one_value_number = comparable_one_value
                                    .clone()
                                    .parse()
                                    .expect("Failed to parse");
                                comparable_two_value_number = comparable_two_value
                                    .clone()
                                    .parse()
                                    .expect("Failed to parse");
                            }

                            _ => {
                                println!(
                                    "{}: Undesired Types: {} {}. Code: {}",
                                    "Error".red(),
                                    comparable_one_type.cyan(),
                                    comparable_two_type.cyan(),
                                    "HRK-MRKM-1Q84".magenta()
                                );
                            }
                        }

                        if comparable_one_value_number < comparable_two_value_number {
                            stack = executable_runner(confirmed_executable_tokens.clone(), stack);
                        } else {
                            stack = executable_runner(else_executable_tokens.clone(), stack);
                        }
                    }

                    _ => {
                        println!(
                            "{}: Unknown Operator: {}",
                            "Error".red(),
                            operator.magenta()
                        );
                        std::process::exit(1)
                    }
                }
            }
        }
    }
    return stack;
}
