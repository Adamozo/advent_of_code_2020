use super::ex15::SIZE;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::collections::HashMap;

pub struct Day15VersionB;

impl DaySolver for Day15VersionB {
    type Output = u32;

    const INFO: DayInfo = <super::ex15::Day15VersionA as DaySolver>::INFO
        .copy_with_different_variant_name("hash map");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut turns: HashMap<u32, usize> = HashMap::new();
        let start_value = insert_init_values(&mut turns, _s)?;

        let res = (turns.len() + 1..SIZE).fold(start_value, |new_num, turn_num| {
            match turns.get(&new_num) {
                None => {
                    turns.insert(new_num, turn_num);
                    0
                },
                Some(previous_turn) => {
                    let next_value = (turn_num - previous_turn) as u32;
                    turns.insert(new_num, turn_num);
                    next_value
                },
            }
        });

        Ok(res)
    }
}

fn insert_init_values(turns: &mut HashMap<u32, usize>, input: &str) -> anyhow::Result<u32> {
    let mut last = 0;
    for (turn_num, value) in input.split(',').enumerate() {
        last = value.parse::<u32>()?;
        turns.insert(last, turn_num + 1);
    }
    Ok(turns.remove_entry(&last).unwrap().0)
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
        Day15VersionB::solution(s).unwrap()
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day15VersionB::solve_default_file().unwrap(), 436)
    }
}
