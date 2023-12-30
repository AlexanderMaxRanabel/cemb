mod external;
mod forever;
mod if_expr;
mod input;
mod memory_management;
mod printline;
mod while_loop;

use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

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
                        let (var_name, type_indicator, var_type, value_indicator) = (
                            tokens[1],
                            tokens[2],
                            tokens[3],
                            tokens[4],
                        );

                        if type_indicator == "::" {
                            if value_indicator == "=" {
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
                                                        println!("Usage: let str :: String = 'Hello world'");
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
                                                    let metadata: String = format!(
                                                        "{} {} {}",
                                                        var_name, var_type, value
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
                                                    let metadata: String = format!(
                                                        "{} {} {}",
                                                        var_name, var_type, value
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
                            } else {
                                println!(
                                    "{}: value Indicator Syntax error: {}",
                                    "Error".red(),
                                    value_indicator
                                );
                                println!("{}: let x :: Int = 42", "Usage".yellow().bold());
                                std::process::exit(1);
                            }
                        } else {
                            println!(
                                "{}: Type Indicator Syntax error: {}",
                                "Error".red(),
                                type_indicator
                            );
                            println!("{}: let x :: Int = 42", "Usage".yellow().bold());
                            std::process::exit(1);
                        }
                    }

                    "printline" => {
                        printline::printline(tokens, stack.clone());
                    }

                    "if" => {
                        stack = if_expr::if_expr(tokens.clone(), stack.clone());
                    }

                    "external" => {
                        external::external(tokens);
                    }

                    "forever" | "loop" => {
                        stack = forever::forever(tokens.clone(), stack.clone());
                    }

                    "dealloc_full_stack" => {
                        stack = memory_management::dealloc_full_stack(stack);
                    }

                    "dealloc_certain_element" => {
                        stack = memory_management::dealloc_certain_element(stack, tokens.clone());
                    }

                    "while" => {
                        stack = while_loop::while_loop(tokens.clone(), stack.clone());
                    }

                    "//" => {
                        continue;
                    }

                    _ => {
                        println!("{}: Unknown Keyword: {}", "Error".red(), tokens[0]);
                        std::process::exit(0)
                    }
                }
            }
        }
    } else {
        println!("{}: No file provided", "Error".red());
    }
    Ok(())
}
