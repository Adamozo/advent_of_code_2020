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
        if line.len() == 0 {
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
    if temp.len() > 0 {
        let field = &temp[0..3];
        return interesting_fields.iter().any(|f| *f==field);
    }

    false
}

fn count_valid_passports(passports: &Vec<String>) -> io::Result<usize> {
    let res = passports
        .iter()
        .filter(|e| {
            e.split(" ")
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
