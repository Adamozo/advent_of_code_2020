use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day4;

impl DaySolver for Day4 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_4", "data_files/ex4.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut mapper = 0;
        let mut counter: usize = 0;

        for line in _s.lines() {
            if line.is_empty() {
                if mapper == 127 {
                    counter += 1;
                }

                mapper = 0;
            } else {
                let m: i16 = line
                    .split(' ')
                    .map(|e| e.split_once(':').unwrap().0)
                    .map(|key| match key {
                        "byr" => 1,
                        "iyr" => 2,
                        "eyr" => 4,
                        "hgt" => 8,
                        "hcl" => 16,
                        "ecl" => 32,
                        "pid" => 64,
                        "cid" => 0,
                        _ => unreachable!(),
                    })
                    .sum();
                mapper += m;
            }
        }

        if mapper == 127 {
            counter += 1;
        }

        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day4::solve_default_file().unwrap(), 2)
    }
}
