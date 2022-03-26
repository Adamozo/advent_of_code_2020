use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Ex3Error {
    #[error("not allowed char occured")]
    InvalidChars,

    #[error("unacceptable line len")]
    WrongLen,

    #[error("to low number of lines in given file (expected {expected:?}, found {found:?})")]
    NotEnaughtLines { expected: usize, found: usize },
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_line(line: &str, expexted_len: usize) -> Result<String, Ex3Error> {
    let cuted: &str = line.trim_start().trim_end();

    // check if line has proper len
    if cuted.len() != expexted_len {
        return Err(Ex3Error::WrongLen);
    }

    // check if unexpected chars occured
    if cuted.chars().filter(|c| *c != '.' && *c != '#').count() > 0 {
        return Err(Ex3Error::InvalidChars);
    }

    Ok(cuted.to_string())
}

pub fn count_trees<P>(path: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    let mut index: usize = 0;
    let mut skip_to_line: usize = 0;
    let mut trees_num: u32 = 0;
    let mut curr_line: usize = 0;

    let board_width: usize = 11;
    let board_height: usize = 11;
    let step: usize = 3;

    for (line_num, line) in (read_lines(path)?).enumerate() {
        let line = line?;
        match process_line(&line, board_width) {
            Ok(p) => {
                curr_line = line_num % board_height;
                if line_num > skip_to_line || skip_to_line == 0 {
                    if p.chars().nth(index).unwrap() == '#' {
                        trees_num += 1;
                    }
                    index += step;

                    if index >= board_width {
                        index = index % board_width;
                        skip_to_line = line_num + board_height;
                    }

                    if (line_num + 1) % board_height == 0 {
                        break;
                    }
                }
            },
            Err(err) => {
                eprintln! {"{} -> Error: {}", line, err};
                break;
            },
        }
    }
    if curr_line + 1 < board_height {
        eprintln! {"Error: {}", Ex3Error::NotEnaughtLines{expected: board_height, found: curr_line+1}};
    }

    Ok(trees_num)
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    println!("couted trees: {}", count_trees(path)?);
    Ok(())
}
