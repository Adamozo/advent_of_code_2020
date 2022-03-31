use anyhow::{Context, Result};
use advent_of_code::*;

fn main() -> Result<()>{
    let filename = "data_files/ex2_passwords.txt";
    ex4::run(filename).with_context(|| format!("Failed to read data from {}", filename))
}
