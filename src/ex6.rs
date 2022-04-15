use std::collections::HashSet;
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

fn count_answers<P>(path: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut questions: HashSet<char> = HashSet::new();
    let mut counter: usize = 0;

    for line in read_lines(path)? {
        let line = line?;
        if line.is_empty() {
            counter += questions.len();
            questions = HashSet::new();
        } else {
            for question in line.chars() {
                questions.insert(question);
            }
        }
    }

    counter += questions.len();

    Ok(counter)
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let passports = count_answers(path)?;
    println!("{:?}", passports);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex4_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_run_file_exists() {
        assert!(!run("data_files/ex6.txt").is_err())
    }
}
