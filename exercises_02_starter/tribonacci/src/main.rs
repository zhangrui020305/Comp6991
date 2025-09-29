use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    // TODO: complete this function!
    let n = match size {
        Ok(v) => v,
        Err(_) => return Err((TribonacciError(error_msg))),
    };

    if n == 0 {
        println!("[]");
        println!("Sum = 0");
        return Ok(());
    }

    let mut tribs: Vec<u64> = Vec::new();
    tribs.push(1);
    if n > 1 { tribs.push(1); }
    if n > 2 { tribs.push(1); }

    for i in 3..n {
        let next = tribs[i - 1] + tribs[i - 2] + tribs[i - 3];
        tribs.push(next);
    }
    println!("Values: {:?}", tribs);

    Ok(())
}
