use crate::{external, if_expr, input, printline};

use colored::*;

pub fn runner(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
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
                                        if string_first_part == "cemb.input" {
                                            let value = input::get_input();
                                            let metadata: String =
                                                format!("{} {} {}", var_name, var_type, value);
                                            stack.push(metadata);
                                        } else {
                                            if let Some(&string_last_part) = tokens.last() {
                                                if string_first_part.starts_with("'")
                                                    && string_last_part.ends_with("'")
                                                {
                                                    let value: String = tokens[5..].join(" ");
                                                    let metadata: String = format!(
                                                        "{} {} {}",
                                                        var_name, var_type, value
                                                    );
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
                                }

                                "Int" => {
                                    if let Some(&value) = tokens.get(5) {
                                        if value == "cemb.input" {
                                            let inputed_value = input::get_input();
                                            if let Ok(_number) = inputed_value.parse::<i64>() {
                                                let metadata: String = format!(
                                                    "{} {} {}",
                                                    var_name, var_type, inputed_value
                                                );
                                                stack.push(metadata);
                                            } else {
                                                println!(
                                                    "{} {}",
                                                    "Not a piece of valid integer: ".red(),
                                                    value
                                                );
                                                std::process::exit(1);
                                            }
                                        } else {
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
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }

                                "Float" => {
                                    if let Some(&value) = tokens.get(5) {
                                        if value == "cemb.input" {
                                            let inputed_value = input::get_input();
                                            if let Ok(_number) = inputed_value.parse::<f64>() {
                                                let metadata: String = format!(
                                                    "{} {} {}",
                                                    var_name, var_type, inputed_value
                                                );
                                                stack.push(metadata);
                                            } else {
                                                println!(
                                                    "{} {}",
                                                    "Not a piece of valid float: ".red(),
                                                    value
                                                );
                                                std::process::exit(1);
                                            }
                                        } else {
                                            if let Ok(_number) = value.parse::<f64>() {
                                                let metadata: String =
                                                    format!("{} {} {}", var_name, var_type, value);
                                                stack.push(metadata);
                                            } else {
                                                println!(
                                                    "{} {}",
                                                    "Not a piece of valid float: ".red(),
                                                    value
                                                );
                                                std::process::exit(1);
                                            }
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
            printline::printline(tokens.clone(), stack.clone());
        }

        "external" => {
            external::external(tokens.clone());
        }

        "if" => {
            stack = if_expr::if_expr(tokens.clone(), stack.clone());
        }

        _ => {
            println!(
                "{}: Unknown Keyword for While loop: {}",
                "Error".red(),
                tokens[0]
            );
            std::process::exit(1);
        }
    }

    return stack;
}

pub fn while_loop(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    if let Some(&iterator_name) = tokens.get(1) {
        if let Some(&bound) = tokens.get(2) {
            if let Some(&value) = tokens.get(3) {
                if let Some(&mode) = tokens.get(4) {
                    if let Some(&rate) = tokens.get(5) {
                        let while_loop_executable_code: Vec<&str> = tokens[6..].to_vec();
                        if let Ok(mut parsed_value) = value.parse::<i64>() {
                            if let Ok(parsed_bound) = bound.parse::<i64>() {
                                if let Ok(parsed_rate) = rate.parse::<i64>() {
                                    match mode {
                                        "+" => {
                                            while parsed_value < parsed_bound {
                                                stack = runner(
                                                    while_loop_executable_code.clone(),
                                                    stack.clone(),
                                                );
                                                parsed_value += parsed_rate;
                                            }
                                        }

                                        "-" => {
                                            while parsed_value > parsed_bound {
                                                stack = runner(
                                                    while_loop_executable_code.clone(),
                                                    stack.clone(),
                                                );
                                                parsed_value -= parsed_rate;
                                            }
                                        }

                                        _ => {
                                            println!(
                                                "{}: Unknown Iterator Mode: {}",
                                                "Error".red(),
                                                mode
                                            );
                                            std::process::exit(1);
                                        }
                                    }

                                    let metadata: String = format!(
                                        "{} {} {}",
                                        iterator_name,
                                        "Int",
                                        parsed_value.to_string()
                                    );
                                    stack.push(metadata);
                                } else {
                                    println!(
                                        "{}: Not a valid piece of Integer: {}",
                                        "Error".red(),
                                        rate
                                    );
                                    std::process::exit(1);
                                }
                            } else {
                                println!(
                                    "{}: Not a valid piece of Integer: {}",
                                    "Error".red(),
                                    bound
                                );
                                std::process::exit(1);
                            }
                        } else {
                            println!("{}: Not a valid piece of Integer: {}", "Error".red(), value);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
    return stack;
}
