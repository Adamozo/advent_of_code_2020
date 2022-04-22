use anyhow::Context;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Ex3Error {
    #[error("not allowed char occured")]
    InvalidChar,

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
    if cuted.chars().any(|c| c != '.' && c != '#') {
        return Err(Ex3Error::InvalidChar);
    }

    Ok(cuted.to_string())
}

fn count_trees<P>(
    path: P,
    board_width: usize,
    board_height: usize,
    step: usize,
) -> anyhow::Result<u32>
where
    P: AsRef<Path>,
{
    let mut index: usize = 0;
    let mut skip_to_line: usize = 0;
    let mut trees_num: u32 = 0;
    let mut curr_line: usize = 0;

    for (line_num, line) in (read_lines(path)?).enumerate() {
        let line = line?;
        let p = process_line(&line, board_width)
            .with_context(|| format!("line content: {} (line={})", line, line_num))?;

        curr_line = line_num % board_height;

        if line_num > skip_to_line || skip_to_line == 0 {
            if p.chars().nth(index).unwrap() == '#' {
                trees_num += 1;
            }
            index += step;

            if index >= board_width {
                index %= board_width;
                skip_to_line = line_num + board_height;
            }

            if (line_num + 1) % board_height == 0 {
                break;
            }
        }
    }
    if curr_line + 1 < board_height {
        return Err(anyhow::anyhow!(
            "{}",
            Ex3Error::NotEnaughtLines {
                expected: board_height,
                found:    curr_line + 1,
            }
        ));
    }

    Ok(trees_num)
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    println!("couted trees: {}", count_trees(path, 11, 11, 3)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("..#.##.....", 20 => Err(Ex3Error::WrongLen); "wrong len (to short))")]
    #[test_case("..#.##.....", 2 => Err(Ex3Error::WrongLen); "wrong len (to long))")]
    #[test_case("..#.##.....", 11 => Ok("..#.##.....".to_string()); "ok len")]
    #[test_case("  ..#.##.....", 11 => Ok("..#.##.....".to_string()); "trim left")]
    #[test_case("..#.##.....   ", 11 => Ok("..#.##.....".to_string()); "trim right")]
    #[test_case("  ..#.##.....  ", 11 => Ok("..#.##.....".to_string()); "trim both")]
    #[test_case(".*#.##.....", 11 => Err(Ex3Error::InvalidChar); "invalid char")]
    fn test_is_valid(s: &str, expexted_len: usize) -> Result<String, Ex3Error> {
        process_line(s, expexted_len)
    }

    #[test]
    fn test_count_trees_no_file() {
        assert!(count_trees("aaa", 1, 1, 1).is_err())
    }

    #[test]
    fn test_count_trees() {
        assert_eq!(
            count_trees("data_files/ex3_given_example.txt", 11, 11, 3).unwrap(),
            7
        );
        assert_eq!(
            count_trees("data_files/ex3_given_example.txt", 11, 11, 0).unwrap(),
            3
        );
        assert!(count_trees("data_files/ex3_given_example.txt", 11, 110, 0).is_err());
    }
}
