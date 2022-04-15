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

fn get_index(filed: &str) -> i16
{
    match filed {
        "byr" => 0, 
        "iyr" => 1,
        "eyr" => 2, 
        "hgt" => 3, 
        "hcl" => 4, 
        "ecl" => 5, 
        "pid" => 6,
        _ => -1
    }
}

fn count_valid_passports<P>(path: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut mapper:[bool; 7] = [false, false, false, false, false, false, false];
    let mut counter: usize = 0;

    for line in read_lines(path)?{
        let line = line?;
        if line.is_empty(){
            if !mapper.iter_mut().any(|e| !(*e)){
                counter += 1;
            }

            mapper = [false, false, false, false, false, false, false];
        }

        else{
            for l in line.split(' '){
                let to_check = l.split(':').next().unwrap();
                let index = get_index(to_check);

                if index != -1{
                    mapper[index as usize] = !mapper[index as usize];
                }
            }
        }
    }

    if !mapper.iter_mut().any(|e| !(*e)){
        counter += 1;
    }

    Ok(counter)
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
}
