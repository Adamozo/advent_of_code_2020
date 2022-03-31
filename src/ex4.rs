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
    let mut passports: Vec<String> = vec!["".to_owned()];
    
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

fn count_valid_passports(passports: &[String]) -> usize {
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
    res
}

pub fn run<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let passports = load_data(path)?;
    println!("{:?}", count_valid_passports(&passports));

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aaa" => false ; "field not interesting")]
    #[test_case("byr" => true ; "field byr interesting")]
    #[test_case("iyr" => true ; "field iyr interesting")]
    #[test_case("eyr" => true ; "field eyr interesting")]
    #[test_case("hgt" => true ; "field hgt interesting")]
    #[test_case("hcl" => true ; "field hcl interesting")]
    #[test_case("ecl" => true ; "field ecl interesting")]
    #[test_case("pid" => true ; "field pid interesting")]
    fn test_ex4_is_field_interesting(f: &str) -> bool{
        is_field_interesting(f)
    }

    #[test]
    fn test_ex4_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_run_file_exists() {
        assert!(run("data_files/ex4.txt").is_err() == false)
    }

    #[test]
    fn test_ex4_count_valid_passports(){
        let valid: [String; 2] = ["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm".to_string()];

        assert_eq!(count_valid_passports(&valid), 2);
    }

    #[test]
    fn test_ex4_count_invalid_passports(){
        let invalid: [String; 2] = [
         "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in".to_string(), "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string()];

        assert_eq!(count_valid_passports(&invalid), 0);
    }
}
