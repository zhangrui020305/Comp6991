use std::io::{self, Write};
fn main() {
    print!("What is your name? ");

    let max_name_len = 100;
    let mut name = String::new();

    io::stdout().flush().unwrap(); // flush prompt

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    if name.len() >= max_name_len {
        name.truncate(max_name_len - 1);
    }

    let name = name.trim_end();

    if name.is_empty() {
        println!("No name entered :(, goodbye.");
        std::process::exit(1); // exit with code 1
    } else {
        println!("Hello, {}, nice to meet you!", name);
    }
}
