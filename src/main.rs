use advent_of_code::*;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    ex13::run().with_context(|| format!("Error in {}", "ex11.rs"))
}
