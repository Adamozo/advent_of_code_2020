use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day9;

impl DaySolver for Day9 {
    type Output = u16;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_9", "data_files/ex9.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        match solve(_s, 5) {
            SolveResult::NoInvalidElement => unreachable!(),
            SolveResult::InvalidElement(element) => Ok(element)
        }
    }
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

#[derive(Debug)]
pub enum SolveResult {
    NoInvalidElement,
    InvalidElement(u16),
}

fn solve(pattern: &str, preamble_size: usize) -> SolveResult {
    let input: Vec<u16> = pattern
        .lines()
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
}
