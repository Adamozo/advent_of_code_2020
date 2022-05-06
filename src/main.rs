use advent_of_code::*;
use anyhow::{Context, Result};

fn main() -> Result<()> {
<<<<<<< HEAD
    let filename = "data_files/ex6.txt";
    ex6::run(filename).with_context(|| format!("Failed to read data from {}", filename))
=======
    let filename = "data_files/ex8.txt";
    ex8::run(filename).with_context(|| format!("Failed to read data from {}", filename))
>>>>>>> ex8
}
