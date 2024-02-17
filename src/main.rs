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
                        let (var_name, var_type) = (tokens[1], tokens[3]);

                        if var_name.starts_with("(") {
                            //The Unexpected behaviour Sector
                            if var_name.starts_with("(") {
                                let raw_var_name_list: Vec<_> = tokens
                                    .iter()
                                    .skip_while(|&&c| c != "(")
                                    .skip(1)
                                    .take_while(|&&c| c != ")")
                                    .collect();

                                let raw_var_value_list: Vec<_> = tokens
                                    .iter()
                                    .skip_while(|&&c| c != "[")
                                    .skip(1)
                                    .take_while(|&&c| c != "]")
                                    .collect();

                                let var_name_list: Vec<&str> =
                                    raw_var_name_list.iter().map(|&&s| s).collect();

                                let mut var_value_list: Vec<&str> =
                                    raw_var_value_list.iter().map(|&&s| s).collect();

                                println!("{:?}", var_value_list);

                                let bulk_type: &str;

                                if let Some(index) = tokens.iter().position(|&token| token == "::")
                                {
                                    if let Some(next_element) = tokens.iter().nth(index + 1) {
                                        bulk_type = next_element;
                                    } else {
                                        println!("{}: No Element After {}", "Error".red(), "::");
                                        std::process::exit(1);
                                    }
                                } else {
                                    println!("{}: {} Did not found", "Error".red(), "::");
                                    std::process::exit(1);
                                }

                                match bulk_type {
                                    "BulkStr" => { 
                                        let mut full_values: Vec<String> = Vec::new();
                                        let mut full_string = String::new();

                                        for val in var_value_list.iter() {
                                            if *val == "," {
                                                if !full_string.is_empty() {
                                                    full_values.push(full_string.clone());
                                                    full_string.clear();
                                                }
                                            } else {
                                                if !full_string.is_empty() {
                                                    full_string.push(' ');
                                                }
                                                full_string.push_str(val);
                                            }
                                        }

                                        if !full_string.is_empty() {
                                            full_values.push(full_string);
                                        }

                                        println!("{:?}", full_values);

                                        for (name, value) in var_name_list.iter().zip(full_values.iter()) {
                                            println!("{}", value);
                                            let metadata: String = format!("{} {} {}", name, "String", value);
                                            println!("{}", metadata);
                                            stack.push(metadata);
                                        }

                                        var_value_list.clear();
                                    }

                                    "BulkInt" => {
                                        for (name, value) in
                                            var_name_list.iter().zip(var_value_list.iter())
                                        {
                                            if let Ok(_number) = value.parse::<i64>() {
                                                let metadata: String =
                                                    format!("{} {} {}", name, "Int", value);
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
                                        var_value_list.clear();
                                    }

                                    "BulkFloat" => {
                                        for (name, value) in
                                            var_name_list.iter().zip(var_value_list.iter())
                                        {
                                            if let Ok(_number) = value.parse::<f64>() {
                                                let metadata: String =
                                                    format!("{} {} {}", name, "Float", value);
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
                                        var_value_list.clear();
                                    }

                                    _ => {
                                        println!("{}: Unknown Type: {}", "Error".red(), bulk_type);
                                        std::process::exit(1);
                                    }
                                }
                            } else {
                                println!("{}: In Bulk Declaration variable name list must start and end with: {}. Yours is: {}", "Error".red(), "()", var_name);
                                std::process::exit(1);
                            }
                        } else {
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
                                    std::process::exit(1);
                                }
                            }
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
