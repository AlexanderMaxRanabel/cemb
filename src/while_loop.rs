use crate::{external, if_expr, printline};

use colored::*;

pub fn runner(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    match tokens[0] {
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
                                                stack = runner(while_loop_executable_code.clone(), stack.clone());
                                                parsed_value += parsed_rate;
                                            }
                                        }

                                        "-" => {
                                            while parsed_value > parsed_bound {
                                                stack = runner(while_loop_executable_code.clone(), stack.clone());
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
