use tribonacci_lib::{compute_tribonacci, utils};

fn main() {
    let shift_size = utils::first_argument();

    match compute_tribonacci(shift_size) {
        Ok((sequence, sum)) => {
            println!("Values: {:?}", sequence);
            println!("\nSum: {}", sum);
        }
        Err(e) => println!("Error: {}", e.0),
    }
}
