use advent_of_code::*;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let filename = "data_files/ex4.txt";
    ex4::run(filename).with_context(|| format!("Failed to read data from {}", filename))
}
