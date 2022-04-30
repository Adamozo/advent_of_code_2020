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

// fn is_sum_of_two(preamble: [&str]) -> bool{
//     let v: Vec<i32> = {
//         let mut v = nums.to_owned();
//         v.sort_unstable();
//         v
//     };

//     let mut res: Vec<i32> = vec![];
//     let mut right_border: usize = v.len() - 1;
//     let mut index: usize = 0;

//     while index != right_border {
//         let value1 = v[index];
//         let value2 = v[right_border];
//         let checked_sum = value1 + value2;

//         if checked_sum <= sum {
//             if checked_sum == sum {
//                 res.push(value1 * value2);
//             }
//             index += 1;
//         } else {
//             right_border -= 1;
//         }
//     }

//     res
// }

#[derive(Debug)]
pub enum SolveResult {
    NoInvalidElement,
    InvalidElement(u16),
}

pub fn solve(pattern: &str, preamble_size: usize) -> SolveResult {
    let res_index = -1;

    let input: Vec<u16> = pattern
        .split("\r\n")
        .map(|f| f.parse::<u16>().unwrap())
        .collect();
    let res = input.iter().enumerate().find(|elem| elem.0 == 200);

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
    let res = solve(data.as_str(), 5);
    println!("Lowest invalid number is is: {:#?}", res);
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_case::test_case;

//     #[test_case("BFFFBBFRRR" => 567)]
//     #[test_case("FBFBBFFRLR" => 357)]
//     #[test_case("FFFBBBFRRR" => 119)]
//     #[test_case("BBFFBBFRLL" => 820)]
//     fn test_extract_seat_num(s: &str) -> u16 {
//         extract_seat_num(s)
//     }

//     #[test_case("data_files/ex5.txt" => 820)]
//     fn test_max_seat_id(s: &str) -> i32 {
//         let res = max_seat_id(s).unwrap();
//         res
//     }

//     #[test]
//     fn test_ex5_run_no_file() {
//         assert!(run("aaa").is_err())
//     }

//     #[test]
//     fn test_ex5_run_file_exists() {
//         assert!(!run("data_files/ex5.txt").is_err())
//     }
// }
