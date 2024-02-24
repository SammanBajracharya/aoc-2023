use std::env;

use trebuchet::{extract_content, get_sum};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents: String = extract_content(&args);

    let sum: u16 = get_sum(contents.as_str());

    println!("Sum : {}", sum);
}
