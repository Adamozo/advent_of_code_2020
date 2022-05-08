/*
    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBF RLL: row 102, column 4, seat ID 820.
    row * 8 + column = seat id
*/

use std::fs::File;
use std::io::BufRead;
use std::io::{self};
use std::path::Path;

const _MAX_BIT_POS: usize = 9; // "FBFBBFFRLR".len() - 1;

fn _extract_seat_num2(code: &str) -> u32 {
    code.bytes()
        .enumerate()
        .filter(|(_, b)| *b == b'B' || *b == b'R')
        .fold(0u32, |id, (high_bit_pos, _)| {
            id | (1 << (_MAX_BIT_POS - high_bit_pos))
        })
}

fn extract_seat_num(input: &str) -> u16 {
    let base: u16 = 2;
    let cords: (u16, u16) =
        input
            .chars()
            .enumerate()
            .fold((0u16, 0u16), |(column, row), (index, p)| {
                if p == 'B' {
                    (column + base.pow(6 - index as u32), row)
                } else if p == 'R' {
                    (column, row + base.pow(9 - index as u32))
                } else {
                    (column, row)
                }
            });

    cords.0 * 8 + cords.1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn max_seat_id<P>(path: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let mut max_seat_num: i32 = -1;

    for line in read_lines(path)? {
        let coded_seat_num = line?;
        let seat_num = i32::from(extract_seat_num(&coded_seat_num));
        if seat_num > max_seat_num {
            max_seat_num = seat_num;
        }
    }

    Ok(max_seat_num)
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let res = max_seat_id(path)?;
    println!("highest seat ID is: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("BFFFBBFRRR" => 567)]
    #[test_case("FBFBBFFRLR" => 357)]
    #[test_case("FFFBBBFRRR" => 119)]
    #[test_case("BBFFBBFRLL" => 820)]
    fn test_extract_seat_num(s: &str) -> u16 {
        extract_seat_num(s)
    }

    #[test_case("data_files/ex5.txt" => 820)]
    fn test_max_seat_id(s: &str) -> i32 {
        let res = max_seat_id(s).unwrap();
        res
    }

    #[test]
    fn test_ex5_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex5_run_file_exists() {
        assert!(!run("data_files/ex5.txt").is_err())
    }
}
