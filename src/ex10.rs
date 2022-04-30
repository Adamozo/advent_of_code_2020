use std::fs;
use std::io::{self};
use std::ops::ControlFlow::{Break, Continue};
use std::path::Path;
use thiserror::Error;

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

pub fn connect_adapters(adapters: &str) -> Result<u64, AdaptersConnectError> {
    let mut res: Vec<u16> = adapters
        .split("\r\n")
        .map(|f| f.parse::<u16>().unwrap())
        .collect();

    res.sort_unstable();

    let connected = res
        .iter()
        .try_fold((0u16, 0u16, 0u16), |acc, x| match *x - acc.2 {
            1 => Continue((acc.0 + 1, acc.1, *x)),
            2 => Continue((acc.0, acc.1, *x)),
            3 => Continue((acc.0, acc.1 + 1, *x)),
            _ => Break(AdaptersConnectError::ToBigDifference(acc.2, *x)),
        });

    match connected {
        Continue(ok) => Ok((ok.0 * ok.1).into()),
        Break(err) => Err(err),
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

    #[test_case("data_files/ex10.txt" => Ok(198))]
    #[test_case("data_files/ex10_error.txt" => Err(AdaptersConnectError::ToBigDifference(49, 200)))]
    fn test_extract_seat_num(s: &str) -> Result<u64, AdaptersConnectError> {
        let data = get_data(s).unwrap();
        connect_adapters(data.as_str())
    }

    #[test]
    fn test_ex10_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex10_run_file_exists() {
        assert!(!run("data_files/ex10.txt").is_err())
    }
}
