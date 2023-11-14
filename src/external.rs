use colored::*;
use std::process::Command;

pub fn external(tokens: Vec<&str>) -> String {
    let mut stdout: String = "".to_string();
    
    if let Some(&external_casm_file) = tokens.get(1) {
        let cemb_run = Command::new("cemb")
            .arg(external_casm_file)
            .output()
            .expect("Failed to run cemb");

        stdout = String::from_utf8_lossy(&cemb_run.stdout).to_string();
        let stderr = String::from_utf8_lossy(&cemb_run.stderr);

        if !cemb_run.status.success() {
            println!("{}: Failed: {}", "Error".red(), stderr.magenta());
            std::process::exit(1);
        }
    }
    return stdout;
}
