use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day22;

const INPUT_SECTION_DELIMETER: &str = "\n\n";

type Card = u8;

impl DaySolver for Day22 {
    type Output = u16;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_22", "data_files/ex22.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (p1_input, p2_input) = _s.split_once(INPUT_SECTION_DELIMETER).unwrap();

        let mut player1 = p1_input.parse::<Player>().unwrap();
        let mut player2 = p2_input.parse::<Player>().unwrap();

        let mut p1_card: Card;
        let mut p2_card: Card;

        let res: u16 = loop {
            // get value of first elements
            p1_card = player1.get_card().unwrap();
            p2_card = player2.get_card().unwrap();

            // game logic
            if p1_card > p2_card {
                player1.put_card(p1_card);
                player1.put_card(p2_card);
            } else {
                player2.put_card(p2_card);
                player2.put_card(p1_card);
            }

            // check if game ended
            if !player1.has_cards() {
                break player2.count_result();
            } else if !player2.has_cards() {
                break player1.count_result();
            }
        };

        Ok(res)
    }
}

#[derive(Debug)]
struct Player {
    cards: VecDeque<Card>,
}

impl Player {
    fn get_card(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    fn put_card(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }

    fn count_result(&self) -> u16 {
        let deck_size = self.cards.len();
        self.cards
            .iter()
            .enumerate()
            .map(|(index, value)| (((deck_size - index) as u8) * (*value)) as u16)
            .sum()
    }
}

impl FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: VecDeque<Card> = s
            .lines()
            .skip(1)
            .map(|card| card.parse::<u8>().unwrap())
            .collect();

        Ok(Player { cards })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex22_count_result() {
        let cards: VecDeque<Card> = VecDeque::from([3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        let player = Player { cards };
        assert_eq!(player.count_result(), 306)
    }

    #[test]
    fn ex22_deck_from_str() {
        let result: VecDeque<Card> = VecDeque::from([9, 2, 6, 3, 1]);
        let input = r#"Player 1:
9
2
6
3
1"#;
        assert_eq!(input.parse::<Player>().unwrap().cards, result)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day22::solve_default_file().unwrap(), 306)
    }
}
