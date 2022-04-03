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

fn count_valid_passports<P>(path: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut container: String = String::new();
    let mut counter: usize = 0;

    let last_empty_string_iter = vec![Ok("".into())].into_iter();
    for line in read_lines(path)?.chain(last_empty_string_iter) {

    for line in read_lines(path)? {
        let line = line?;
        if line.is_empty() {
            if is_passport_valid(&container){
                counter += 1;
                container = " ".to_owned();
            }
        } else {
            container += " ";
            container += &*line;
        }
    }

    Ok(counter)
}

fn is_field_interesting(f: &str) -> u32 {
    let interesting_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if !f.is_empty() {
        let field = &f[0..3];  // split_once
        if interesting_fields.iter().any(|f| *f == field) {
            return 1;
        }
    }

    0
}

fn is_passport_valid(passport: &str) -> bool {
    let mut res = 0;
    for p in passport.split(' '){
        if is_field_interesting(&p) + res >= 7{
            return true;
        }

        else if is_field_interesting(&p)==1{
            res += 1;
        }
    }
    false
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let passports = count_valid_passports(path)?;
    println!("{:?}", passports);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aaa" => 0 ; "field not interesting")]
    #[test_case("byr" => 1 ; "field byr interesting")]
    #[test_case("iyr" => 1 ; "field iyr interesting")]
    #[test_case("eyr" => 1 ; "field eyr interesting")]
    #[test_case("hgt" => 1 ; "field hgt interesting")]
    #[test_case("hcl" => 1 ; "field hcl interesting")]
    #[test_case("ecl" => 1 ; "field ecl interesting")]
    #[test_case("pid" => 1 ; "field pid interesting")]
    fn test_ex4_is_field_interesting(f: &str) -> u32 {
        is_field_interesting(f)
    }

    #[test]
    fn test_ex4_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_run_file_exists() {
        assert!(!run("data_files/ex4.txt").is_err())
    }

    #[test]
    fn test_ex4_count_valid_passports() {
        assert_eq!(count_valid_passports("data_files/ex4.txt").unwrap(), 2);
    }

    #[test_case("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in" => false; "passport invalid")]
    #[test_case("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm" => true ; "passport valid")]
    #[test_case("" => false ; " emptypassport invalid")]
    fn test_ex4_is_passport_valid(passport: &str) -> bool{
        is_passport_valid(passport)
    }
}
