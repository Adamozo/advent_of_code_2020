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

    #[error("to low number of lines in given file")]
    NotEnaughtLines,
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

/*
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
*/

pub fn count_trees<P>(path: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    let mut index = 0;
    let mut current_line = 0;
    let board_width: usize = 11;
    let mut trees_num: u32 = 0;

    for (line_num, line) in (read_lines(path)?).enumerate() {
        let line = line?;
        match process_line(&line, board_width) {
            Ok(p) => {
                if p.chars().nth(index % board_width).unwrap() == '#' {
                    trees_num += 1;
                }
                index += 1;
                current_line += 1;
            }
            Err(err) => {
                eprintln! {"{} -> Error: {}", line, err};
                break;
            }
        }
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
