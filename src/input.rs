use std::io;

pub fn get_input() -> String {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    value = value.lines().collect::<String>();
    return value;
}
