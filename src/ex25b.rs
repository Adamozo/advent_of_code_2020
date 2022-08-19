use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

const SUBJECT_NUMBER: u64 = 7;

pub struct Day25b;

impl DaySolver for Day25b {
    type Output = u64;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_25", "data_files/ex25.txt", "perf");

    fn solution(s: &str) -> anyhow::Result<Self::Output> {
        let public_keys: Vec<u64> = s
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .collect();

        let mut pk_1 = 1_u64;
        let mut pk_2 = 1_u64;
        let mut pk_base = 1_u64;

        loop {
            pk_1 = calc_public_key(pk_1, public_keys[1]);
            pk_2 = calc_public_key(pk_2, public_keys[0]);
            pk_base = calc_public_key(pk_base, SUBJECT_NUMBER);

            // println!("pk_1={:>8}, pk_2={:>8}, pk_base={}", pk_1, pk_2, pk_base);

            if pk_base == public_keys[0] {
                break Ok(pk_1);
            }

            if pk_base == public_keys[1] {
                break Ok(pk_2);
            }
        }
    }
}
// -----------------------------------------------------------------------------

fn calc_public_key(mut public_key: u64, subject_number: u64) -> u64 {
    public_key *= subject_number as u64;
    public_key %= 20201227;
    public_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day25b::solve_default_file().unwrap(), 14897079)
    }
}
