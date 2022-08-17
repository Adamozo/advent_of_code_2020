use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::str;
use std::char::from_digit;
use std::str::FromStr;

/*
We have:
      123456789|label
      258647391|next(label)
      012345678|idx of next; next[idx] contains label (label=idx+1)

      next[i] contains label of the cup next to the cup with label i+1

      e.g.: next[0] contains 2; 2 is the label of the cup next to  the cup with label 1 (1=0+1)

      You can read this as follows [ column wise label->next(label) ] :
      2 is next to 1
      5 is next to 2
      8 is next to 3
      6 is next to 4
      ..............
      1 is next to 9
*/

pub struct Day23b;

type Cup = u32;
type Cups = Vec<Cup>;


impl DaySolver for Day23b {
    type Output = String;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_23", "data_files/ex23.txt", "new data struct");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let mut game_engine = _s.parse::<GameEngine>()?;

        Ok(game_engine.run_game())
    }
}

#[derive(PartialEq, Debug)]
struct GameEngine {
    cups: Cups,
    picked_cups: Cups,
    current_cup: Cup,
}

impl GameEngine {
    fn run_game(&mut self) -> String {
        let mut destination_cup;

        for _i in 1..=100 {
            self.pick_three_after_current_cup();
            destination_cup = self.get_destination();
            self.push_three_after_cup(&destination_cup);

            self.current_cup = self.get_next_cup(&self.current_cup);
        }

        self.get_result()
    }

    fn get_next_cup(&self, cup: &Cup) -> Cup {
        self.cups[(cup-1) as usize]
    }

    fn pick_three_after_current_cup(&mut self) {
        self.picked_cups[0] = self.get_next_cup(&self.current_cup);
        self.picked_cups[1] = self.get_next_cup(&self.picked_cups[0]);
        self.picked_cups[2] = self.get_next_cup(&self.picked_cups[1]);

        // change next cup for selected_cup
        self.cups[(self.current_cup-1) as usize] = self.get_next_cup(&self.picked_cups[2]);
    }

    fn get_destination(&mut self) -> Cup {
        let mut destination = self.current_cup - 1;
        
        let min_cup = self.min_cup();
        let max_cup = self.max_cup();

        loop {
            if destination < min_cup {
                return max_cup;
            } else {
                if !self.picked_cups.contains(&destination) {
                    return destination;
                }

                destination -= 1;
            }
        }
    }

    fn push_three_after_cup(&mut self, destination_cup: &Cup) {
        let first_cup = self.get_next_cup(destination_cup);

        self.cups[(destination_cup-1) as usize] = self.picked_cups[0];
        self.cups[(self.picked_cups[0]-1) as usize] = self.picked_cups[1];
        self.cups[(self.picked_cups[1]-1) as usize] = self.picked_cups[2];
        self.cups[(self.picked_cups[2]-1) as usize] = first_cup;
    }

    fn max_cup(&self) -> Cup {
        let mut current = self.get_next_cup(&self.current_cup);
        let mut max = current;
        loop {
            current = self.get_next_cup(&current);

            if current == self.current_cup {
                break max;
            }

            if current > max {
                max = current;
            }

        }
    }

    fn min_cup(&self) -> Cup {
        let mut current = self.get_next_cup(&self.current_cup);
        let mut min = current;
        loop {
            current = self.get_next_cup(&current);

            if current == self.current_cup {
                break min;
            }

            if current < min {
                min = current;
            }

        }
    }

    fn get_result(&self) -> String {
        let mut res: String = "".to_string();

        let one = self.cups.iter().position(|value| *value == 1).unwrap();
        let mut pointer = self.get_next_cup(&self.cups[one]);

        loop {
            res.push(from_digit(pointer, 10).unwrap());
            pointer = self.get_next_cup(&pointer);

            if pointer == 1 {
                break;
            }
        }

        res
    }
}

impl FromStr for GameEngine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cups: Cups = vec![0;9];

        let mut input = s.chars().map(|cup| cup.to_digit(10).unwrap() as Cup);
        let mut previous = input.next().unwrap();
        let first = previous;

        for cup in input {
            cups[(previous-1) as usize] = cup;
            previous = cup;
        }

        cups[(previous-1) as usize] = first;

        let picked_cups: Cups = vec![0,0,0];
        let current_cup = first;

        Ok(GameEngine {
            cups,
            picked_cups,
            current_cup
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day23b::solve_default_file().unwrap(), "67384529".to_owned())
    }

    #[test]
    fn ex23_game_engine_from_str() {
        let cups: Cups = vec![2, 5, 8, 6, 4, 7, 3, 9, 1];
        let picked_cups: Cups = vec![0;3];
        let current_cup: Cup = 3;
        let res = GameEngine {
            cups,
            picked_cups,
            current_cup
        };
        assert_eq!("389125467".parse::<GameEngine>().unwrap(), res)
    }

    #[test]
    fn ex23_get_result() {
        let game_engine = "389125467".parse::<GameEngine>().unwrap();
        assert_eq!(game_engine.get_result(), "25467389".to_string())
    }

    #[test]
    fn ex23_max_cup() {
        let game_engine = "3125467".parse::<GameEngine>().unwrap();
        assert_eq!(game_engine.max_cup(), 7)
    }

    #[test]
    fn ex23_push_three_after_cup() {
        let mut game_engine = "389125467".parse::<GameEngine>().unwrap();
        game_engine.pick_three_after_current_cup();

        game_engine.push_three_after_cup(&2);
        assert_eq!(game_engine.cups, vec![5, 8, 2, 6, 4, 7, 3, 9, 1])
    }

    #[test]
    fn ex23_get_destination() {
        let mut game_engine = "389125467".parse::<GameEngine>().unwrap();

        assert_eq!(game_engine.get_destination(), 2)
    }
}
