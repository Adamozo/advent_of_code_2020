use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

const MODULO: u128 = 20201227;

pub struct Day25;

impl DaySolver for Day25 {
    type Output = u128;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_25", "data_files/ex25.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let mut iter = _s.lines().map(|linse| linse.parse::<u128>().unwrap());

        let mut public_key1 = iter.next().unwrap();
        let public_key2 = iter.next().unwrap();

        let mut loop_size_key2 = (0..u32::MAX)
            .find(|loop_size| 7_u128.pow(*loop_size) % MODULO == public_key2)
            .unwrap();

        let mut result = 1;

        // Algorytm szybkiego potęgowania
        while loop_size_key2 != 0 {
            if loop_size_key2 % 2 != 0 {
                result = result * public_key1 % MODULO;
            }
            public_key1 = public_key1.pow(2) % MODULO;
            loop_size_key2 /= 2;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day25::solve_default_file().unwrap(), 14897079)
    }
}
