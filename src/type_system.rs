//type_system.rs

use colored::*;

pub fn integer_type(tokens: Vec<&str>, mut stack: Vec<String>, var_name: &str, var_type: &str) {
    if let Some(&value) = tokens.get(5) {
        if let Ok(_number) = value.parse::<i32>() {
            let metadata: String = format!("{}, {}, {}", var_name, var_type, value);
            stack.push(metadata);
            println!("{:?}", stack);
        } else {
            println!("{} {}", "Not a piece of valid integer: ".red(), value);
        }
    }
}

pub fn float_type(tokens: Vec<&str>, mut stack: Vec<String>, var_name: &str, var_type: &str) {
    if let Some(&value) = tokens.get(5) {
        if let Ok(_number) = value.parse::<f64>() {
            let metadata: String = format!("{}, {}, {}", var_name, var_type, value);
            stack.push(metadata);
            println!("{:?}", stack)
        } else {
            println!("{} {}", "Not a piece of float: ".red(), value);
        }
    }
}

pub fn string_type(tokens: Vec<&str>, mut stack: Vec<String>, var_name: &str, var_type: &str) {
    if let Some(&string_first_part) = tokens.get(5) {
        if let Some(&string_last_part) = tokens.last() {
            if string_first_part.starts_with("'") && string_last_part.ends_with("'") {
                let value: String = tokens[5..].join(" ");
                let metadata: String = format!("{}, {}, {}", var_name, var_type, value);
                stack.push(metadata);
            } else {
                println!("Usage: let str :: String = 'Hello world'");
                std::process::exit(0);
            }
        }
    }
}

pub fn char_type(tokens: Vec<&str>, mut stack: Vec<String>, var_name: &str, var_type: &str) {
    if let Some(&value) = tokens.get(5) {
        if value.len() == 1 {
            let metadata: String = format!("{}, {}, {}", var_name, var_type, value);
            stack.push(metadata);
        } else {
            println!("{}: {}", "Not a valid piece of char due to length".red(), value);
        }
    }
}