use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day1;

impl DaySolver for Day1 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_1", "data_files/ex1.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let nums = _s.lines().map(|line| line.parse::<i32>().unwrap()).collect();
        let result_vec = check_sum(nums, 2020);
        Ok(result_vec[0]) // return first as it is in task but checks for all pairs
    }
}

fn check_sum(nums: Vec<i32>, sum: i32) -> Vec<i32> {
    let v: Vec<i32> = {
        let mut v = nums;
        v.sort_unstable();
        v
    };

    let mut res: Vec<i32> = vec![];
    let mut right_border: usize = v.len() - 1;
    let mut index: usize = 0;

    while index != right_border {
        let value1 = v[index];
        let value2 = v[right_border];
        let checked_sum = value1 + value2;

        if checked_sum <= sum {
            if checked_sum == sum {
                res.push(value1 * value2);
            }
            index += 1;
        } else {
            right_border -= 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1721, 979, 366, 299, 675, 1456], 2020 => vec![514579]; "test from website")]
    #[test_case(vec![2040, -20, 366, 299, 675, 1456], 2020 => vec![-40800]; "negative value in array")]
    #[test_case(vec![-2040, 20, 366, 299, 675, 1456], -2020 => vec![-40800]; "negative sum result")]
    #[test_case(vec![1721, 979, 366, 0, 675, 1456], 2020 => Vec::<i32>::new(); "lack of matched pair")]
    #[test_case(vec![2, 2018, 20, 2000], 2020 => vec![4036, 40000]; "2 matching pairs")]
    #[test_case(vec![2, 2018, 20, 2000, 40], 2020 => vec![4036, 40000]; "2 matching pairs one exstra")]
    fn test_check_sum(nums: Vec<i32>, sum: i32) -> Vec<i32> {
        check_sum(nums, sum)
    }
}
