use std::io::{self, BufRead};

fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                if text.contains(pattern){
                    println!("{}", text)
                }
            }
            Err(err) => eprintln!("error reading line: {}", err),
        }
    }
}
