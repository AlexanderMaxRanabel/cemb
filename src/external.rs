use colored::*;
use std::process::Command;

pub fn external(tokens: Vec<&str>) -> String {
    println!(
        "{}: Experimental. Assumes cemb is in the path",
        "Warning".yellow().bold()
    );

    let mut result = String::new();
    if let Some(&file) = tokens.get(1) {
        let output = Command::new("cemb")
            .arg(file)
            .output()
            .expect("Failed to run command");

        if output.status.success() {
            result = String::from_utf8_lossy(&output.stdout).to_string();
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to execute command. error: {}", stderr);
        }
    }

    return result;
}
