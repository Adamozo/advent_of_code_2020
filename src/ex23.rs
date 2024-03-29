use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::char::from_digit;
use std::str;
use std::str::FromStr;

pub struct Day23;

type Cup = u32;
type Cups = Vec<Cup>;

impl DaySolver for Day23 {
    type Output = String;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_23", "data_files/ex23.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let mut game_engine = _s.parse::<GameEngine>().unwrap();

        Ok(game_engine.run_game())
    }
}

#[derive(PartialEq, Debug)]
struct GameEngine {
    cups:        Cups,
    min_cup:     Cup,
    picked_cups: Cups,
}

impl GameEngine {
    fn run_game(&mut self) -> String {
        let mut current_cup = self.cups[0];
        let mut destination_cup;

        for _ in 1..=100 {
            self.pick_three_after_cup(&current_cup);
            destination_cup = self.get_destination(&current_cup);
            self.push_three_after_cup(&destination_cup);

            current_cup = self.get_cup_after_cup(current_cup);
        }

        self.get_result()
    }

    fn pick_three_after_cup(&mut self, selected_cup: &Cup) {
        for _i in 0..=2 {
            self.picked_cups[_i] = self
                .cups
                .remove(self.calculate_next_location(self.get_cup_location(selected_cup)));
        }
    }

    fn calculate_next_location(&self, checked_location: usize) -> usize {
        (checked_location + 1) % self.cups.len()
    }

    fn get_cup_location(&self, checked_cup: &Cup) -> usize {
        self.cups
            .iter()
            .enumerate()
            .find(|(_, value)| *value == checked_cup)
            .map(|(index, _)| index)
            .unwrap()
    }

    fn get_destination(&mut self, current_cup: &Cup) -> Cup {
        let mut destination = current_cup - 1;
        let min_cup = self.min_cup();

        loop {
            if destination < min_cup {
                return self.max_cup();
            } else {
                if !self.picked_cups.contains(&destination) {
                    return destination;
                }

                destination -= 1;
            }
        }
    }

    fn push_three_after_cup(&mut self, selected_cup: &Cup) {
        let insert_start_location = self.get_cup_location(selected_cup) + 1;

        for (offset, cup) in self.picked_cups.iter().enumerate() {
            self.cups.insert(insert_start_location + offset, *cup);
        }
    }

    fn max_cup(&self) -> Cup {
        *self.cups.iter().max().unwrap()
    }

    fn min_cup(&self) -> Cup {
        *self.cups.iter().min().unwrap()
    }

    fn get_cup_after_cup(&self, cup: Cup) -> Cup {
        let position = (self
            .cups
            .iter()
            .position(|element| *element == cup)
            .unwrap()
            + 1)
            % self.cups.len();
        self.cups[position]
    }

    fn get_result(&mut self) -> String {
        let one = self.cups.iter().position(|value| *value == 1).unwrap();
        let mut res = self.cups.split_off(one);
        let _ = res.remove(0);

        res.append(&mut self.cups);

        res.iter()
            .map(|num| from_digit(*num, 10).unwrap())
            .collect::<String>()
    }
}

impl FromStr for GameEngine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups: Cups = s
            .chars()
            .map(|cup| cup.to_digit(10).unwrap() as Cup)
            .collect();
        let min_cup: Cup = *cups.iter().min().unwrap();
        let picked_cups: Cups = vec![0, 0, 0];

        Ok(GameEngine {
            cups,
            min_cup,
            picked_cups,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day23::solve_default_file().unwrap(), "67384529".to_owned())
    }

    #[test]
    fn ex23_game_engine_from_str() {
        let cups: Cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let min_cup: Cup = 1;
        let picked_cups: Cups = vec![0; 3];
        let res = GameEngine {
            cups,
            min_cup,
            picked_cups,
        };
        assert_eq!("389125467".parse::<GameEngine>().unwrap(), res)
    }

    #[test]
    fn ex23_get_result() {
        let mut game_engine = "389125467".parse::<GameEngine>().unwrap();
        assert_eq!(game_engine.get_result(), "25467389".to_string())
    }

    #[test_case(3=>8)]
    #[test_case(6=>7)]
    #[test_case(7=>3)]
    fn ex23_get_cup_after_cup(cup: Cup) -> Cup {
        let game_engine = "389125467".parse::<GameEngine>().unwrap();
        game_engine.get_cup_after_cup(cup)
    }

    #[test]
    fn ex23_max_cup() {
        let game_engine = "3125467".parse::<GameEngine>().unwrap();
        assert_eq!(game_engine.max_cup(), 7)
    }

    #[test]
    fn ex23_push_three_after_cup() {
        let cups: Cups = vec![3, 8, 9, 1, 2, 5];
        let min_cup: Cup = 1;
        let picked_cups: Cups = vec![4, 6, 7];
        let mut game_engine = GameEngine {
            cups,
            min_cup,
            picked_cups,
        };
        game_engine.push_three_after_cup(&5);
        assert_eq!(game_engine.cups, vec![3, 8, 9, 1, 2, 5, 4, 6, 7])
    }

    #[test]
    fn ex23_get_destination() {
        let cups: Cups = vec![3, 8, 9, 1, 2, 5];
        let min_cup: Cup = 1;
        let picked_cups: Cups = vec![4, 6, 7];
        let mut game_engine = GameEngine {
            cups,
            min_cup,
            picked_cups,
        };

        assert_eq!(game_engine.get_destination(&3), 2);
        assert_eq!(game_engine.get_destination(&1), 9)
    }
}
