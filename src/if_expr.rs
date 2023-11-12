use crate::{
    memory_management::*,
    printline::*,
    external::*
};

use colored::*;

pub fn if_expr(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    if let Some(&comparable_one_address_raw) = tokens.get(1) {
    if let Some(&comparable_two_address_raw) = tokens.get(3) {
        if let Some(&operator) = tokens.get(2) {
            let comparable_one_address: usize = comparable_one_address_raw.parse::<usize>().expect("Failed to convert");
            let comparable_two_address: usize = comparable_two_address_raw.parse::<usize>().expect("Failed to convert");

            let comparable_one_metadata: Vec<String> = (stack[comparable_one_address].clone()).split_whitespace().map(|s: &str| s.to_string()).collect();
            let comparable_two_metadata: Vec<String> = (stack[comparable_two_address].clone()).split_whitespace().map(|s: &str| s.to_string()).collect();
            
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

            let else_executable_tokens: Vec<&str> = else_executable_tokens_double.iter().map(|&&s| s).collect();
            let confirmed_executable_tokens: Vec<&str> = confirmed_executable_tokens_double.iter().map(|&&s| s).collect();

            match operator {
                "==" => {
                    if comparable_one_value == comparable_two_value {
                        match confirmed_executable_tokens[0] {
                            "printline" => {
                                printline(confirmed_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, confirmed_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        }
                    } else {
                        match else_executable_tokens[0] {
                            "printline" => {
                                printline(else_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, else_executable_tokens);                                                        
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), else_executable_tokens[0].magenta());
                                std::process::exit(1);
                            },
                        }
                    }
                },  

                "!=" => {
                    if comparable_one_value != comparable_two_value {
                        match confirmed_executable_tokens[0] {
                            "printline" => {
                                printline(confirmed_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, confirmed_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        }
                    } else {
                        match else_executable_tokens[0] {
                            "printline" => {
                                printline(else_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, else_executable_tokens)
                            }

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), else_executable_tokens[0].magenta());
                                std::process::exit(1);
                            },
                        }
                    }
                },

                ">" => {
                    let mut comparable_one_value_number: f64 = 0.0;
                    let mut comparable_two_value_number: f64 = 0.0;

                    match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                        ("Float", "Float") | ("Int", "Int") => {
                            comparable_one_value_number = comparable_one_value.clone().parse().expect("Failed to parse");
                            comparable_two_value_number = comparable_two_value.clone().parse().expect("Failed to parse");
                        },

                        _ => {
                            println!("{}: Undesired Types: {} {}. Code: {}", "Error".red(), comparable_one_type.cyan(), comparable_two_type.cyan(), "HRK-MRKM-1Q84".magenta());
                        },
                    }

                    if comparable_one_value_number > comparable_two_value_number {
                        match confirmed_executable_tokens[0] {
                            "printline" => {
                                printline(confirmed_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, confirmed_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        } 
                    } else {
                        match else_executable_tokens[0] {
                            "printline" => {
                                printline(else_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, else_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), else_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        }
                    }
                },

                "<" => {
                    let mut comparable_one_value_number: f64 = 0.0;
                    let mut comparable_two_value_number: f64 = 0.0;

                    match (comparable_one_type.as_str(), comparable_two_type.as_str()) {
                        ("Float", "Float") | ("Int", "Int") => {
                            comparable_one_value_number = comparable_one_value.clone().parse().expect("Failed to parse");
                            comparable_two_value_number = comparable_two_value.clone().parse().expect("Failed to parse");
                        },

                        _ => {
                            println!("{}: Undesired Types: {} {}. Code: {}", "Error".red(), comparable_one_type.cyan(), comparable_two_type.cyan(), "HRK-MRKM-1Q84".magenta());
                        },
                    }

                    if comparable_one_value_number < comparable_two_value_number {
                        match confirmed_executable_tokens[0] {
                            "printline" => {
                                printline(confirmed_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, confirmed_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), confirmed_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        } 
                    } else {
                        match else_executable_tokens[0] {
                            "printline" => {
                                printline(else_executable_tokens.clone(), stack.clone());
                            },

                            "dealloc_full_stack" => {
                                stack = dealloc_full_stack(stack);
                            },

                            "dealloc_certain_element" => {
                                stack = dealloc_certain_element(stack, else_executable_tokens);
                            },

                            _ => {
                                println!("{}: Unknown Keyword: {}", "Error".red(), else_executable_tokens[0].magenta());
                                std::process::exit(1);
                            }, 
                        }
                    }
                },
                
                    _ => {
                    
                        println!("{}: Unknown Operator: {}", "Error".red(), operator.magenta());
                        std::process::exit(1)
                    },
                }
            }
        }
    }
    return stack;
}