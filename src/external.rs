use colored::*;
use std::process::Command;

pub fn external(tokens: Vec<&str>) -> String {
    println!("{}: Experimental. Assumes cemb is in the path", "Warning".yellow().bold());
    let mut result: String = "".to_string();
    
    if let Some(&external_casm_file) = tokens.get(1) {
        let cemb_run = Command::new("cemb")
            .arg(external_casm_file)
            .output()
            .expect("Failed to run cemb");

        if !cemb_run.status.success() {
            result = String::from_utf8_lossy(&cemb_run.stderr).to_string();
        } else {
            result = String::from_utf8_lossy(&cemb_run.stdout).to_string();
        }
    }
    return result;
}
