use advent_of_code::*;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let filename = "data_files/ex10.txt";
    ex10::run(filename).with_context(|| format!("Failed to read data from {}", filename))
}
