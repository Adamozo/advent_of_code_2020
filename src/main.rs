mod ex2;
//mod ex1;
use anyhow::{Context, Result};

fn main() -> Result<()>{
    let filename = "data_files/ex2_passwords.txt";
    ex2::run(filename).with_context(|| format!("Failed to read data from {}", filename))
}
