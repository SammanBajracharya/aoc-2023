use std::env;
use cube::{extract_content, contain_cube};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String = extract_content(&args);

    let sum: u16 = contain_cube(&contents);

    println!("Sum: {}", sum);
}
