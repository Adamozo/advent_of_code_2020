use std::fs;
use std::io::{self};
use std::ops::ControlFlow::{Break, Continue};
use std::path::Path;
use thiserror::Error;

use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day10;

impl DaySolver for Day10 {
    type Output = u64;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_10", "data_files/ex10.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> { 
        connect_adapters(_s)
    }
}

fn get_data<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

#[derive(Error, Debug, PartialEq)]
pub enum AdaptersConnectError {
    #[error("To big difference between adaper `{0}` and `{1}`")]
    ToBigDifference(u16, u16),
}

pub fn connect_adapters(adapters: &str) -> anyhow::Result<u64> {
    let mut res: Vec<u16> = adapters
        .lines()
        .map(|f| f.parse::<u16>().unwrap())
        .collect();

    res.sort_unstable();

    let connected =
        res.iter().try_fold(
            (0u16, 0u16, 0u16),
            |(diff_1, diff_3, prev_val), val| match *val - prev_val {
                1 => Continue((diff_1 + 1, diff_3, *val)),
                2 => Continue((diff_1, diff_3, *val)),
                3 => Continue((diff_1, diff_3 + 1, *val)),
                _ => Break(AdaptersConnectError::ToBigDifference(prev_val, *val)),
            },
        );

    match connected {
        Continue(ok) => Ok((ok.0 * (ok.1 + 1)).into()),
        Break(err) => Err(anyhow::anyhow!("{}", err)),
    }
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data = get_data(path)?;
    let res = connect_adapters(data.as_str())?;
    println!("Result of multiplication is: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test_case("data_files/ex10.txt" => Ok(220))]
    // #[test_case("data_files/ex10_error.txt" => Err(AdaptersConnectError::ToBigDifference(49, 200)))]
    // fn test_extract_seat_num(s: &str) -> Result<u64, AdaptersConnectError> {
    //     let data = get_data(s).unwrap();
    //     connect_adapters(data.as_str())
    // }

    

    #[test]
    fn test_ex10_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex10_run_file_exists() {
        assert!(!run("data_files/ex10.txt").is_err())
    }
}
