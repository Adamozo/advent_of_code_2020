use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

use itertools::{Itertools, Permutations};
use std::fmt;
use std::str::FromStr;
use text_io::scan;

pub struct Day20;

type TilesPossibilities = Vec<Vec<Tile>>;
type Board = Vec<Tile>;

impl DaySolver for Day20 {
    type Output = u128;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_20", "data_files/ex20.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let grid = _s.parse::<Grid>()?;
        let res = grid.solve().unwrap();

        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileStage {
    Base,
    VerticallyFlipped,
    HorizontallyFlipped,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RotationState {
    R0,
    R90,
    R180,
    R270,
}

fn get_next_stage(current_stage: &TileStage) -> Option<TileStage> {
    use TileStage::*;

    match current_stage {
        Base => Some(VerticallyFlipped),
        VerticallyFlipped => Some(HorizontallyFlipped),
        HorizontallyFlipped => None,
    }
}

fn get_next_rotation(current_rotation: &RotationState) -> Option<RotationState> {
    use RotationState::*;

    match current_rotation {
        R0 => Some(R90),
        R90 => Some(R180),
        R180 => Some(R270),
        R270 => None,
    }
}

struct Grid {
    domains: TilesPossibilities,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Tile> = _s
            .split("\n\n")
            .map(|tile| tile.parse::<Tile>().unwrap())
            .collect();

        let domains = tiles.iter().map(|tile| tile.get_permutations()).collect();

        Ok(Grid { domains })
    }
}

impl Grid {
    fn placement_permutations(&self) -> Permutations<std::slice::Iter<'_, Vec<Tile>>> {
        self.domains.iter().permutations(self.domains.len())
    }

    fn solve(&self) -> Option<u128> {
        for grid in self.placement_permutations() {
            match recur_solver(
                &grid,
                &mut (0..9).into_iter().map(|_| Tile::new()).collect(),
                0,
            ) {
                None => {
                    continue;
                },
                Some(res) => {
                    return Some(res);
                },
            }
        }

        None
    }
}

fn recur_solver(
    domanins: &Vec<&Vec<Tile>>, //  TilesPossibilities with &
    board: &mut Board,
    current_index: usize,
) -> Option<u128> {
    for tile in domanins[current_index] {
        if can_insert(board, current_index, tile) {
            board[current_index] = tile.clone();

            if current_index == board.len() - 1 {
                return Some(board[0].id * board[2].id * board[6].id * board[8].id);
            }

            match recur_solver(domanins, board, current_index + 1) {
                None => {
                    continue;
                },
                Some(res) => {
                    return Some(res);
                },
            }
        }
    }

    None
}

fn can_insert(board: &[Tile], index: usize, tile: &Tile) -> bool {
    match index {
        0 => true,
        1 => board[0].right == tile.left,
        2 => board[1].right == tile.left,
        3 => board[0].bottom == tile.top,
        4 => board[1].bottom == tile.top && board[3].right == tile.left,
        5 => board[2].bottom == tile.top && board[4].right == tile.left,
        6 => board[3].bottom == tile.top,
        7 => board[4].bottom == tile.top && board[6].right == tile.left,
        8 => board[5].bottom == tile.top && board[7].right == tile.left,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: u128,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
    stage: TileStage,
    rotation_state: RotationState,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    /*
     -> 0001111110

        1           0
        1           1
        1           1
        1           1
        1           1
        1           1
        1           1
     ^  1           1 ^
     |  1           1 |

     -> 1001111110

    */

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let res = _s
            .lines()
            .enumerate()
            .fold(
                Tile::new(),
                |mut tile, (line_number, line)| match line_number {
                    0 => {
                        scan!(line.bytes() => "Tile {}:", tile.id);
                        tile
                    },
                    1 => {
                        tile.top = number_from_line(line);

                        let (left, right) = calculate_borders_change(line, line_number);
                        tile.left += left;
                        tile.right += right;

                        tile
                    },
                    10 => {
                        tile.bottom = number_from_line(line);

                        let (left, right) = calculate_borders_change(line, line_number);
                        tile.left += left;
                        tile.right += right;

                        tile
                    },
                    _ => {
                        let (left, right) = calculate_borders_change(line, line_number);
                        tile.left += left;
                        tile.right += right;
                        tile
                    },
                },
            );

        Ok(res)
    }
}

fn calculate_borders_change(line: &str, line_number: usize) -> (u16, u16) {
    let mut letters = line.chars();
    let (first, last) = (letters.next().unwrap(), letters.last().unwrap());

    let mut left = 0;
    let mut right = 0;

    if first == '#' {
        left += 2_u16.pow((line_number - 1).try_into().unwrap());
    }

    if last == '#' {
        right += 2_u16.pow((line_number - 1).try_into().unwrap());
    }

    (left, right)
}

impl Tile {
    fn new() -> Self {
        Self {
            id: 0,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
            stage: TileStage::Base,
            rotation_state: RotationState::R0,
        }
    }

    fn rotate(&mut self) {
        let top_new = roatate_binary(self.right);
        let bottom_new = roatate_binary(self.left);
        let left_new = self.top;
        let right_new = self.bottom;

        self.top = top_new;
        self.bottom = bottom_new;
        self.left = left_new;
        self.right = right_new;

        match get_next_rotation(&self.rotation_state) {
            Some(rotation) => {
                self.rotation_state = rotation;
            },

            None => {
                self.rotation_state = RotationState::R0;
            },
        }
    }

    fn flip_horizontal(&mut self) {
        (self.left, self.right) = (self.right, self.left);
        self.top = roatate_binary(self.top);
        self.bottom = roatate_binary(self.bottom);
    }

    fn flip_vertical(&mut self) {
        (self.top, self.bottom) = (self.bottom, self.top);
        self.right = roatate_binary(self.right);
        self.left = roatate_binary(self.left);
    }

    fn get_permutations(&self) -> Vec<Tile> {
        self.clone().permutations()
    }

    fn permutations(&mut self) -> Vec<Tile> {
        let mut res: Vec<Tile> = Vec::new();

        loop {
            res.push(self.clone());

            self.rotate();

            if self.rotation_state == RotationState::R0 {
                match get_next_stage(&self.stage) {
                    None => {
                        break;
                    },

                    Some(next_stage) => {
                        if next_stage == TileStage::HorizontallyFlipped {
                            self.flip_horizontal();
                        } else if next_stage == TileStage::VerticallyFlipped {
                            self.flip_horizontal();
                            self.flip_vertical();
                        }

                        self.stage = next_stage;
                    },
                }
            }
        }

        res
    }
}

fn number_from_line(line: &str) -> u16 {
    line.chars().enumerate().fold(0, |number, (index, letter)| {
        if letter == '#' {
            number + 2_u16.pow((9 - index).try_into().unwrap())
        } else {
            number
        }
    })
}

fn roatate_binary(number: u16) -> u16 {
    number.reverse_bits() >> 6 // u10 not u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use RotationState::*;
    use TileStage::*;

    #[test]
    fn ex20_data_from_default_file() {
        assert_eq!(Day20::solve_default_file().unwrap(), 20899048083289);
    }

    #[test_case(8 => 64)]
    #[test_case(67 => 776)]
    #[test_case(391 => 902)]
    #[test_case(20 => 160)]
    fn ex20_rotate_binary(num: u16) -> u16 {
        roatate_binary(num)
    }

    #[test_case("#...##.#.." => 564)]
    #[test_case("..##.#..#." => 210)]
    #[test_case("###.##.#.." => 948)]
    #[test_case("...##....." => 96)]
    fn ex20_number_from_line(line: &str) -> u16 {
        number_from_line(line)
    }

    #[test_case(Base => Some(VerticallyFlipped))]
    #[test_case(VerticallyFlipped => Some(HorizontallyFlipped))]
    #[test_case(HorizontallyFlipped => None)]
    fn ex20_get_next_stage(current_stage: TileStage) -> Option<TileStage> {
        get_next_stage(&current_stage)
    }

    #[test_case(R0 => Some(R90))]
    #[test_case(R90 => Some(R180))]
    #[test_case(R180 => Some(R270))]
    #[test_case(R270 => None)]
    fn ex20_get_next_rotation(current_rotation: RotationState) -> Option<RotationState> {
        get_next_rotation(&current_rotation)
    }

    #[test]
    fn ex20_tile_from_str() {
        let res = Tile {
            id: 2311,
            top: 210,
            bottom: 231,
            left: 318,
            right: 616,
            stage: Base,
            rotation_state: R0,
        };
        let input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###"#;

        assert_eq!(input.parse::<Tile>().unwrap(), res);
    }

    #[test]
    fn ex20_tile_rotate() {
        let mut base = Tile {
            id: 2311,
            top: 210,
            bottom: 231,
            left: 318,
            right: 616,
            stage: Base,
            rotation_state: R0,
        };
        let res = Tile {
            id: 2311,
            top: 89,
            bottom: 498,
            left: 210,
            right: 231,
            stage: Base,
            rotation_state: R90,
        };
        base.rotate();

        assert_eq!(base, res);
    }

    #[test]
    fn ex20_flip_horizontal() {
        let mut base = Tile {
            id: 2311,
            top: 210,
            bottom: 231,
            left: 318,
            right: 616,
            stage: Base,
            rotation_state: R0,
        };
        let res = Tile {
            id: 2311,
            top: 300,
            bottom: 924,
            left: 616,
            right: 318,
            stage: Base,
            rotation_state: R0,
        };
        base.flip_horizontal();

        assert_eq!(base, res);
    }

    #[test]
    fn ex20_flip_vertical() {
        let mut base = Tile {
            id: 2311,
            top: 210,
            bottom: 231,
            left: 318,
            right: 616,
            stage: Base,
            rotation_state: R0,
        };
        let res = Tile {
            id: 2311,
            top: 231,
            bottom: 210,
            left: 498,
            right: 89,
            stage: Base,
            rotation_state: R0,
        };
        base.flip_vertical();

        assert_eq!(base, res);
    }
}
