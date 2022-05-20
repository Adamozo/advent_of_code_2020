use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::collections::HashMap;

pub struct Day14;

impl DaySolver for Day14 {
    type Output = u64;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_14", "data_files/ex14.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (set, unset, dict) = prepare_input(_s)?;

        let result: u64 = dict
            .iter()
            .map(|(_, value)| {
                let mut res = *value;
                res |= set;
                res &= unset;
                res as u64
            })
            .sum();

        Ok(result)
    }
}

pub fn prepare_input(_s: &str) -> anyhow::Result<(u64, u64, HashMap<u64, u64>)> {
    let input_lines: Vec<&str> = _s.lines().collect();
    let mask: &str = input_lines[0].split(" = ").nth(1).unwrap();

    let mut memorized: HashMap<u64, u64> = HashMap::new();

    for i in 1..input_lines.len() {
        let splited_line: Vec<&str> = input_lines[i].split(" = ").collect();
        let num = splited_line[1].parse::<u64>()?;
        let key = splited_line[0]
            .split('[')
            .nth(1)
            .unwrap()
            .split(']')
            .nth(0)
            .unwrap()
            .parse::<u64>()?;
        memorized.insert(key, num);
    }

    let (set, unset) = process_mask(mask)?;

    Ok((set, unset, memorized))
}

pub fn process_mask(mask: &str) -> anyhow::Result<(u64, u64)> {
    let binary_base: usize = 2;
    let (set, unset) = mask
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, letter)| *letter == '1' || *letter == '0')
        .fold((0, 0), |(ones, zeros), (counter, letter)| {
            if letter == '1' {
                (ones + binary_base.pow(counter as u32), zeros)
            } else {
                (ones, zeros + binary_base.pow(counter as u32))
            }
        });
    Ok((set as u64, !unset as u64))
}
