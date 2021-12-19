use std::env;
use std::fs;

mod day4;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Parse
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let fname = &args[1];
    let s: String = fs::read_to_string(&fname)?;
    println!("File exists; reading...");

    day4::day4(&s)
}
