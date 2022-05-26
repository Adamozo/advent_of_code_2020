pub struct Day14VariantB;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

impl DaySolver for Day14VariantB {
    type Output = u64;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_14", "data_files/ex14.txt", "two vectors");

    fn solution(s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        use super::ex14::{Instruction, Processor};

        let sum = s
            .lines()
            .filter_map(|line| line.parse::<Instruction>().ok())
            .fold(Processor::default(), |mut processor, instruction| {
                processor.process(instruction);
                processor
            })
            .output_value();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day14VariantB::solve_default_file().unwrap(), 165)
    }
}
