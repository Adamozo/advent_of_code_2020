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

fn load_data<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut passports: Vec<String> = Vec::new();
    passports.push("".to_owned());

    for line in read_lines(path)? {
        let line = line?;
        if line.is_empty() {
            passports.push("".to_owned());
        } else {
            let size: usize = passports.len() - 1;
            passports[size] += " ";
            passports[size] += &*line;
        }
    }

    Ok(passports)
}

fn is_field_interesting(f: &str) -> bool {
    let temp = f.to_string();
    let interesting_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if !temp.is_empty() {
        let field = &temp[0..3];
        return interesting_fields.iter().any(|f| *f==field);
    }

    false
}

fn count_valid_passports(passports: &[String]) -> io::Result<usize> {
    let res = passports
        .iter()
        .filter(|e| {
            e.split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .filter(|c| is_field_interesting(c))
                .count()
                >= 7
        })
        .count();
    Ok(res)
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let passports = load_data(path)?;
    println!("{:?}", count_valid_passports(&passports)?);

    Ok(())
}


#[cfg(test)]
mod tests {
    // use super::*;
    // use test_case::test_case;

    // #[test_case("1-3 a: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'a', passwd: "abcde".to_string()}); "valid webiste 1")]
    // #[test_case("1-3 b: cdefg" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'b', passwd: "cdefg".to_string()}); "valid website 2")]
    // #[test_case("2-9 c: ccccccccc" => Ok(Password{min_number: 2, max_number: 9, checked_char: 'c', passwd: "ccccccccc".to_string()}); "valid website 3")]
    // #[test_case("1-c a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as max_number")]
    // #[test_case("c-3 a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as min_number")]
    // #[test_case("1-3 1: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: '1', passwd: "abcde".to_string()}); "valid num as checked char")]
    // #[test_case("1-3 a: " => Err(PasswordError::CaptureFailed); "invalid lack of password")]
    // fn test_from_str(s: &str) -> Result<Password, PasswordError> {
    //     s.parse::<Password>()
    // }

    // #[test_case("1-3 a: abcde" => Ok(true) ; "valid 1-3 a: abcde")]
    // #[test_case("1-3 b: cdefg" =>  Ok(false)  ; "invalid 1-3 b: cdefg")]
    // #[test_case("2-9 c: ccccccccc" =>  Ok(true)  ; "valid 2-9 c: ccccccccc")]
    // #[test_case("3-1 a: abcde" => Ok(false) ; "invalid 3-1 a: abcde")]
    // #[test_case("1-c a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as max_number")]
    // #[test_case("c-3 a: abcde" => Err(PasswordError::CaptureFailed); "invalid letter as min_number")]
    // fn test_is_valid(s: &str) -> Result<bool, PasswordError> {
    //     let p1 = s.parse::<Password>()?;
    //     Ok(p1.is_valid())
    // }

    // #[test]
    // fn test_run_no_file() {
    //     assert!(run("aaa").is_err())
    // }

    // #[test]
    // fn test_run_file_exists() {
    //     assert!(run("data_files/ex2_passwords.txt").is_err() == false)
    // }
}
