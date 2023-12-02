use crate::{external, if_expr, printline};

use colored::*;

pub fn forever(tokens: Vec<&str>, mut stack: Vec<String>) -> Vec<String> {
    if let Some(&main_keyword) = tokens.get(1) {
        let forever_loop_executable_code: Vec<&str> = tokens[1..].to_vec();
        loop {
            match main_keyword {
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
