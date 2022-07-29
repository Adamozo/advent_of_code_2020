use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day22;

const INPUT_SECTION_DELIMETER: &str = "\n\n";

impl DaySolver for Day22 {
    type Output = u16;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_22", "data_files/ex22.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (p1_input, p2_input) = _s.split_once(INPUT_SECTION_DELIMETER).unwrap();

        let mut player1_deck = deck_from_str(p1_input);
        let mut player2_deck = deck_from_str(p2_input);

        let mut p1_card: u16;
        let mut p2_card: u16;

        let res: u16 = loop {
            // get value of first elements
            p1_card = *player1_deck.first().unwrap();
            p2_card = *player2_deck.first().unwrap();

            // remove first elements
            player1_deck.remove(0);
            player2_deck.remove(0);

            // game logic
            if p1_card > p2_card {
                player1_deck.push(p1_card);
                player1_deck.push(p2_card);
            } else {
                player2_deck.push(p2_card);
                player2_deck.push(p1_card);
            }

            // check if game ended
            if player1_deck.is_empty() {
                break count_result(&player2_deck);
            } else if player2_deck.is_empty() {
                break count_result(&player1_deck);
            }
        };

        Ok(res)
    }
}

fn deck_from_str(player_input: &str) -> Vec<u16> {
    player_input
        .lines()
        .skip(1)
        .map(|card| card.parse::<u16>().unwrap())
        .collect()
}

fn count_result(deck: &Vec<u16>) -> u16 {
    println!("{:?}", deck);
    let deck_size = deck.len();
    deck.iter().enumerate().fold(0, |result, (index, value)| {
        result + ((deck_size - index) as u16) * value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex22_count_result() {
        let input: Vec<u16> = vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1];
        assert_eq!(count_result(&input), 306)
    }

    #[test]
    fn ex22_deck_from_str() {
        let result: Vec<u16> = vec![9, 2, 6, 3, 1];
        let input = r#"Player 1:
9
2
6
3
1"#;
        assert_eq!(deck_from_str(input), result)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day22::solve_default_file().unwrap(), 306)
    }
}
