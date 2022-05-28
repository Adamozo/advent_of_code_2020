use anyhow::*;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day17;

impl DaySolver for Day17 {
    type Output = u32;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_17", "data_files/ex17.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        Ok(5)
    }
}

#[derive(PartialEq, Debug)]
enum CubeState{
    Active,
    Inactive
}

impl TryFrom<char> for CubeState {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == '#' {
            Ok(CubeState::Active)
        } else if value == '.'{
            Ok(CubeState::Inactive)
        }
        else {
            Err(anyhow!("Unknown cube state: {}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    #[test]
    fn ex17_data_from_default_file() {
        assert_eq!(Day17::solve_default_file().unwrap(), 5);
    }

    #[test]
    fn ex17_cube_state_from() {
        assert_eq!(CubeState::try_from('#').unwrap(), CubeState::Active);
        assert_eq!(CubeState::try_from('.').unwrap(), CubeState::Inactive);
        assert!(CubeState::try_from('-').is_err());
    }
}