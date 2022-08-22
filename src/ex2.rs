use lazy_regex::{regex, Lazy, Regex};
use std::str::FromStr;
use thiserror::Error;

use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day2;

impl DaySolver for Day2 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_2", "data_files/ex2.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {

        let res = _s.lines().filter_map(|line| line.parse::<Password>().ok()).count();

        Ok(res)
    }
}

#[derive(PartialEq, Debug, Eq)]
pub struct Password {
    min_number:   u16,
    max_number:   u16,
    checked_char: char,
    passwd:       String,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PasswordError {
    #[error("unable to capture password")]
    CaptureFailed,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        let counter: u16 = self
            .passwd
            .chars()
            .filter(|c| *c == self.checked_char)
            .count() as u16;

        (self.min_number..=self.max_number).contains(&counter)
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    // 1-3 a: abcde
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: &Lazy<Regex> =
            regex!(r"(?P<min>\d+)-(?P<max>\d+) (?P<checked_char>\w{1}): (?P<passwd>\w+)");
        let caps = re.captures(s);

        if let Some(r) = caps {
            Ok(Password {
                min_number:   (&r["min"]).parse::<u16>().unwrap(),
                max_number:   (&r["max"]).parse::<u16>().unwrap(),
                checked_char: (&r["checked_char"]).chars().next().unwrap(),
                passwd:       (&r["passwd"]).to_string(),
            })
        } else {
            Err(PasswordError::CaptureFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'a', passwd: "abcde".to_string()}); "valid webiste 1")]
    #[test_case("1-3 b: cdefg" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'b', passwd: "cdefg".to_string()}); "valid website 2")]
    #[test_case("2-9 c: ccccccccc" => Ok(Password{min_number: 2, max_number: 9, checked_char: 'c', passwd: "ccccccccc".to_string()}); "valid website 3")]
    #[test_case("1-c a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as max_number")]
    #[test_case("c-3 a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as min_number")]
    #[test_case("1-3 1: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: '1', passwd: "abcde".to_string()}); "valid num as checked char")]
    #[test_case("1-3 a: " => Err(PasswordError::CaptureFailed); "invalid lack of password")]
    fn test_from_str(s: &str) -> Result<Password, PasswordError> {
        s.parse::<Password>()
    }

    #[test_case("1-3 a: abcde" => Ok(true) ; "valid 1-3 a: abcde")]
    #[test_case("1-3 b: cdefg" =>  Ok(false)  ; "invalid 1-3 b: cdefg")]
    #[test_case("2-9 c: ccccccccc" =>  Ok(true)  ; "valid 2-9 c: ccccccccc")]
    #[test_case("3-1 a: abcde" => Ok(false) ; "invalid 3-1 a: abcde")]
    #[test_case("1-c a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as max_number")]
    #[test_case("c-3 a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as min_number")]
    fn test_is_valid(s: &str) -> Result<bool, PasswordError> {
        let p1 = s.parse::<Password>()?;
        Ok(p1.is_valid())
    }
}
