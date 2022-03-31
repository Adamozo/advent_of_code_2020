use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_data<P>(path: P) -> anychow::Result<()> 
where 
    P: AsRef<Path>,
{
    for line in read_lines(path)?{
        let line = line?;
        match line.parse::<Password>(){
            Ok(p) => println!("{:?}: is_valid == {}", line, p.is_valid()),
            Err(err) => eprintln! {"{:?} -> Error: {}", line, err},
        }
    }
    
    Ok(())
}

pub fn run<P>(path: P) -> anychow::Result<()> 
where 
    P: AsRef<Path>,
{
    load_data(path)
}