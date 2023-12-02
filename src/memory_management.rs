use colored::*;

pub fn dealloc_full_stack(mut stack: Vec<String>) -> Vec<String> {
    stack.clear();
    stack.shrink_to_fit();

    return stack;
}

pub fn dealloc_certain_element(mut stack: Vec<String>, tokens: Vec<&str>) -> Vec<String> {
    if let Some(&element_to_remove) = tokens.get(1) {
        let element_to_remove: usize = element_to_remove
            .parse()
            .expect("Failed to convert to usize");
        if (element_to_remove) < stack.len() {
            stack.remove(element_to_remove);
        } else {
            println!(
                "{} {}",
                " Error: Cannot remove element because it does not exist".red(),
                element_to_remove.to_string()
            );
            std::process::exit(0);
        }
    }

    return stack;
}
