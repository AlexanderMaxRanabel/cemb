use crate::{
    printline,
    external
};

use colored::*;

pub fn forever(tokens: Vec<&str>, stack: Vec<String>) {
    if let Some(&main_keyword) = tokens.get(1) {
        let forever_loop_executable_code: Vec<&str> = tokens[1..].to_vec();
                            
            loop {
                match main_keyword {
                    "printline" => {
                         printline::printline(forever_loop_executable_code.clone(), stack.clone());
                    },

                    "external" => {
                        external::external(forever_loop_executable_code.clone());
                    },

                    _ => {
                        println!("{}: Unknown Keyword for Forever loop: {}", "Error".red(), main_keyword);
                        std::process::exit(1);
                    },
            }
        }    
    } 
}
