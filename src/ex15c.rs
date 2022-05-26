use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day15VersionC;

impl DaySolver for Day15VersionC {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file_and_variant(
        "day_15",
        "data_files/ex15.txt",
        "vector with capacity",
    );

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut turns: Vec<u32> = Vec::with_capacity(2020);
        let start_value = insert_init_values(&mut turns, _s)?;

        let res = (turns.len() + 1..2020).fold(start_value, |new_num, _| {
            match turns.iter().rev().position(|&num| num == new_num) {
                None => {
                    turns.push(new_num);
                    0
                },
                Some(index) => {
                    let next_value = (index + 1) as u32;
                    turns.push(new_num);
                    next_value
                },
            }
        });

        Ok(res)
    }
}

fn insert_init_values(turns: &mut Vec<u32>, input: &str) -> anyhow::Result<u32> {
    for value in input.split(',') {
        turns.push(value.parse::<u32>()?);
    }

    Ok(turns.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0,3,6" => 436)]
    #[test_case("1,3,2" => 1)]
    #[test_case("2,1,3" => 10)]
    #[test_case("1,2,3" => 27)]
    #[test_case("2,3,1" => 78)]
    #[test_case("3,2,1" => 438)]
    #[test_case("3,1,2" => 1836)]
    fn data_from_exapmles(s: &str) -> u32 {
        Day15VersionC::solution(s).unwrap()
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day15VersionC::solve_default_file().unwrap(), 436)
    }
}
