use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Day16;

type AvailableRanges = Vec<RangeInclusive<u32>>;
type FieldValue = u32;

const INPUT_SECTION_DELIMETER: &str = "\r\n\r\n";

impl DaySolver for Day16 {
    type Output = u32;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_16", "data_files/ex16.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (classes_section, nearby_tickets_section) = {
            let mut sections = _s.split(INPUT_SECTION_DELIMETER);

            (sections.next().unwrap(), {
                sections.next(); // skip "your ticket section"
                sections.next().unwrap()
            })
        };

        let fields_ranges = classes_section.parse::<FieldsRanges>()?;

        let result = nearby_tickets_section
            .lines()
            .skip(1)
            .map(|ticket| fields_ranges.count_ticket_error_rate(ticket))
            .sum();

        Ok(result)
    }
}

struct FieldsRanges {
    available_ranges: AvailableRanges,
}

impl FromStr for FieldsRanges {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let available_ranges: AvailableRanges = s
            .lines()
            .flat_map(|line| {
                line.split_once(": ").unwrap().1.split(" or ").map(|field| {
                    let (start, end) = field.split_once('-').unwrap();
                    start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap()
                })
            })
            .collect();

        Ok(FieldsRanges { available_ranges })
    }
}

impl FieldsRanges {
    fn is_in_any_range(&self, value: &FieldValue) -> bool {
        self.available_ranges
            .iter()
            .any(|range| range.contains(value))
    }

    fn count_ticket_error_rate(&self, line: &str) -> u32 {
        line.split(',')
            .map(|num| num.parse::<u32>().unwrap())
            .filter(|value| !self.is_in_any_range(value))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn ex16_fields_ranges_from_str() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44";

        let result = input.parse::<FieldsRanges>().unwrap();
        assert_eq!(result.available_ranges, vec![1..=3, 5..=7, 6..=11, 33..=44]);
    }

    #[test_case(7 => true)]
    #[test_case(4 => false)]
    fn ex16_is_in_any_range(num: u32) -> bool {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44";

        let ranges = input.parse::<FieldsRanges>().unwrap();
        ranges.is_in_any_range(&num)
    }

    #[test_case("7,3,47" => 0)]
    #[test_case("40,4,50" => 4)]
    #[test_case("55,2,20" => 55)]
    fn ex16_count_error_rate(ticket: &str) -> u32 {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50";

        let ranges = input.parse::<FieldsRanges>().unwrap();
        ranges.count_ticket_error_rate(ticket)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day16::solve_default_file().unwrap(), 71)
    }
}
