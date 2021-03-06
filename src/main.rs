use std::env;
use std::fs;
use std::error::Error;

mod day10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let fname = &args[1];
    let s: String = fs::read_to_string(&fname)?;
    println!("File exists; reading...");

    day10::day10(&s)
}
