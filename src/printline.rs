use colored::*;

pub fn printline(tokens: Vec<&str>, stack: Vec<String>) {
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
                        //printline 2 + 1
                        if tokens.len() > 3 {
                            if let Ok(number_integer_1) = (tokens.get(1).expect("Failed to receive or parse")).parse::<i64>() {
                                if let Ok(number_integer_2) = (tokens.get(3).expect("Failed to receive or parse")).parse::<i64>() {
                                    if let Some(&operator) = tokens.get(2) {
                                        match operator {
                                            "+" => {
                                                let result = number_integer_1 + number_integer_2;
                                                println!("{}", result);
                                            },

                                            "-" => {
                                                let result = number_integer_1 - number_integer_2;
                                                println!("{}", result);
                                            },

                                            "*" => {
                                                let result = number_integer_1 * number_integer_2;
                                                println!("{}", result);
                                            },

                                            "/" => {
                                                let result = number_integer_1 / number_integer_2;
                                                println!("{}", result);
                                            },

                                            _ => {
                                                println!("{}: {}", "Error: Not a valid operator ".red(), operator);
                                            }
                                        }
                                    }
                                } else {
                                    println!("{} {}", "Not an integer:".red(), tokens.get(1).expect("Failed to receive it"));
                                }
                            } else if let Ok(number_float_1) = (tokens.get(1).expect("Failed to receive or parse")).parse::<f64>() {
                                if let Ok(number_float_2) = (tokens.get(3).expect("Failed to receive or parse")).parse::<f64>() {
                                    if let Some(&operator) = tokens.get(2) {
                                        match operator {
                                            "+" => {
                                                let result = number_float_1 + number_float_2;
                                                println!("{}", result);
                                            },

                                            "-" => {
                                                let result = number_float_1 - number_float_2;
                                                println!("{}", result);
                                            },

                                            "*" => {
                                                let result = number_float_1 * number_float_2;
                                                println!("{}", result);
                                            },

                                            "/" => {
                                                let result = number_float_1 / number_float_2;
                                                println!("{}", result);
                                            },

                                            _ => {
                                                println!("{}: {}", "Error: Not a valid operator ".red(), operator);
                                            }
                                        }
                                    }
                                } else {
                                    println!("{} {}", "Not an float:".red(), tokens.get(1).expect("Failed to receive it"));
                                }
                            } else {
                                println!("{}: {}", "Error: Not a valid piece of integer nor float ".red(), (tokens.get(1).expect("Failed to receive it")));
                                std::process::exit(1)
                            }
                        } else {
                            for element in &stack {
                                let metadata_array: Vec<String> = element.split_whitespace().map(|s| s.to_string()).collect();
                                let variable_name = metadata_array[0].clone();
                                if second_token == variable_name {
                                    if let Some(&ref value) = metadata_array.get(2) {
                                        println!("{}", value);
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}
