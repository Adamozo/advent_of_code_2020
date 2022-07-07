use std::fs;
use std::io::{self};
use std::path::Path;

fn get_data<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn is_sum_of_two(mut preamble: Vec<u16>, element: &u16) -> bool {
    preamble.sort_unstable();

    let mut right_border: usize = preamble.len() - 1;
    let mut index: usize = 0;

    while index != right_border {
        let value1: u16 = preamble[index];
        let value2: u16 = preamble[right_border];
        let checked_sum: u16 = value1 + value2;

        if checked_sum <= *element {
            if checked_sum == *element {
                return true;
            }
            index += 1;
        } else {
            right_border -= 1;
        }
    }

    false
}

#[derive(Debug, PartialEq)]
pub enum SolveResult {
    NoInvalidElement,
    InvalidElement(u16),
}

pub fn solve(pattern: &str, preamble_size: usize) -> SolveResult {
    let input: Vec<u16> = pattern
        .split("\r\n")
        .map(|f| f.parse::<u16>().unwrap())
        .collect();
    let res = input
        .iter()
        .enumerate()
        .filter(|elem| elem.0 >= preamble_size)
        .find(|elem| !is_sum_of_two((&input[elem.0 - preamble_size..elem.0]).to_vec(), elem.1));

    match res {
        None => SolveResult::NoInvalidElement,
        Some(elem) => SolveResult::InvalidElement(*elem.1),
    }
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data = get_data(path)?;
    match solve(data.as_str(), 5) {
        SolveResult::InvalidElement(element) => {
            println!("Lowest invalid number is is: {}", element)
        },
        SolveResult::NoInvalidElement => println!("Input does not have invalid element"),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test_case("data_files/ex9.txt" => SolveResult::InvalidElement(127))]
    // #[test_case("data_files/ex9_no_invalid.txt" => SolveResult::NoInvalidElement)]
    // fn test_solve(s: &str) -> SolveResult {
    //     let data = get_data(s).unwrap();
    //     solve(data.as_str(), 5)
    // }

    #[test_case(vec![1,2,3], &10 => false)]
    #[test_case(vec![1,2,3], &5 => true)]
    fn test_is_sum_of_two(preamble: Vec<u16>, element: &u16) -> bool {
        is_sum_of_two(preamble, element)
    }

    #[test]
    fn test_ex9_run_no_file() {
        assert!(run("aaa").is_err())
    }
}
