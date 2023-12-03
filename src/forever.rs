use crate::{external, if_expr, printline};

use colored::*;

pub fn forever(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    if let Some(&main_keyword) = tokens.get(1) {
        let forever_loop_executable_code: Vec<&str> = tokens[1..].to_vec();
        loop {
            match main_keyword {
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

                                        "Int" => {
                                            if let Some(&value) = tokens.get(5) {
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
                                                }
                                            }
                                        }

                                        "Float" => {
                                            if let Some(&value) = tokens.get(5) {
                                                if let Ok(_number) = value.parse::<f64>() {
                                                    let metadata: String = format!(
                                                        "{} {} {}",
                                                        var_name, var_type, value
                                                    );
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
                                                    let metadata: String = format!(
                                                        "{} {} {}",
                                                        var_name, var_type, value
                                                    );
                                                    stack.push(metadata);
                                                } else {
                                                    println!(
                                                        "{}: {}",
                                                        "Not a valid piece of char due to length"
                                                            .red(),
                                                        value
                                                    );
                                                }
                                            }
                                        }

                                        _ => {
                                            println!(
                                                "{} {}",
                                                "Error: Unknown Type: ".red(),
                                                var_type
                                            );
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
                    printline::printline(forever_loop_executable_code.clone(), stack.clone());
                }

                "external" => {
                    external::external(forever_loop_executable_code.clone());
                }

                "if" => {
                    stack = if_expr::if_expr(forever_loop_executable_code.clone(), stack.clone());
                }

                _ => {
                    println!(
                        "{}: Unknown Keyword for Forever loop: {}",
                        "Error".red(),
                        main_keyword
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    return stack;
}
