use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day14VariantB;

impl DaySolver for Day14VariantB {
    type Output = u64;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_14", "data_files/ex14.txt", "two vectors");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        use Instruction::*;
        let mut set = 0;
        let mut unset = 0;

        let mut addresses: Vec<u64> = Vec::new();
        let mut values: Vec<u64> = Vec::new();

        let mut sum: u64 = 0;

        for line in _s.lines() {
            match line.parse::<Instruction>()? {
                Mask(mask) => {
                    (set, unset) = process_mask(&mask)?;
                },
                Mem(address, value) => {
                    if let Some(index) = addresses.iter().position(|&x| x == address) {
                        let mut to_add = value;
                        to_add |= set;
                        to_add &= unset;

                        sum -= values[index];

                        values[index] = to_add;
                        sum += to_add;
                    } else {
                        addresses.push(address);
                        let mut to_add = value;
                        to_add |= set;
                        to_add &= unset;

                        values.push(to_add);

                        sum += to_add;
                    }
                },
            }
        }

        Ok(sum)
    }
}

pub struct Day14VariantA;

impl DaySolver for Day14VariantA {
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

enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use text_io::try_scan;

        let instruction = if s.starts_with("ma") {
            let mask: String;

            try_scan!(s.bytes() => "mask = {}", mask);

            Self::Mask(mask.trim().parse()?)
        } else {
            let address_value: u64;
            let mem_value: u64;

            try_scan!(s.bytes() => "mem[{}] = {}", address_value, mem_value);

            Self::Mem(address_value, mem_value)
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn ex14_process_mask() {
        let result = process_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!((result.0, !result.1), (64, 2));
    }

    #[test]
    fn ex14_process_input() {
        use aoc_utils::read_to_string;
        let dict: HashMap<u64, u64> = HashMap::from([(7, 101), (8, 0)]);
        assert_eq!(
            prepare_input(&read_to_string("data_files/ex14.txt").unwrap()).unwrap(),
            (64, 18446744073709551613, dict)
        );
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day14VariantA::solve_default_file().unwrap(), 165);
        assert_eq!(Day14VariantB::solve_default_file().unwrap(), 165)
    }
}
