use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashSet;

pub struct Day24;

#[derive(Debug, Eq, PartialEq, Hash)]
struct TilePosition {
    column: i32,
    row: i32,
}

impl TilePosition {
    fn new() -> Self {
        Self { column: 0, row: 0 }
    }

    fn shift(&mut self, direction: &[char]) {
        // https://math.stackexchange.com/questions/2254655/hexagon-grid-coordinate-system
        // Axial Coordinates
        match direction {
            ['e'] => {
                self.column += 1;
            },
            ['s', 'e'] => {
                self.row += 1;
            },
            ['n', 'e'] => {
                self.column += 1;
                self.row -= 1;
            },
            ['w'] => {
                self.column -= 1;
            },
            ['s', 'w'] => {
                self.column -= 1;
                self.row += 1;
            },
            ['n', 'w'] => {
                self.row -= 1;
            },
            _ => unreachable!(),
        }
    }
}

impl DaySolver for Day24 {
    type Output = u32;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_24", "data_files/ex24.txt", "base");

    fn solution(s: &str) -> anyhow::Result<<Self>::Output> {
        let black_tiles = s.lines().map(get_tile_position).fold(
            FnvHashSet::default(),
            |mut black_tiles: FnvHashSet<TilePosition>, tile_position| {
                if !black_tiles.remove(&tile_position) {
                    black_tiles.insert(tile_position);
                }

                black_tiles
            },
        );

        Ok(black_tiles.len() as u32)
    }
}

fn get_tile_position(line: &str) -> TilePosition {
    let (_, tile) = line.chars().fold(
        (None, TilePosition::new()),
        |(acc_letter, mut tile), new_letter| {
            if new_letter == 'e' || new_letter == 'w' {
                if let Some(old_letter) = acc_letter {
                    tile.shift(&[old_letter, new_letter]);
                } else {
                    tile.shift(&[new_letter]);
                }

                (None, tile)
            } else {
                (Some(new_letter), tile)
            }
        },
    );

    tile
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sesenwnenenewseeswwswswwnenewsewsw" => TilePosition{column: -3, row: 2})]
    #[test_case("neeenesenwnwwswnenewnwwsewnenwseswesw" => TilePosition{column: 1, row: -3})]
    #[test_case("seswneswswsenwwnwse" => TilePosition{column: -3, row: 3})]
    fn ex24_get_tile_position(line: &str) -> TilePosition {
        get_tile_position(line)
    }

    #[test_case(&['s', 'e'] => TilePosition{column: 0, row: 1})]
    #[test_case(&['n', 'e'] => TilePosition{column: 1, row: -1})]
    #[test_case(&['e'] => TilePosition{column: 1, row: 0})]
    #[test_case(&['s', 'w'] => TilePosition{column: -1, row: 1})]
    #[test_case(&['n', 'w'] => TilePosition{column: 0, row: -1})]
    #[test_case(&['w'] => TilePosition{column: -1, row: 0})]
    fn ex24_tile_shift(direction: &[char]) -> TilePosition {
        let mut tile = TilePosition::new();
        tile.shift(direction);
        tile
    }

    #[test]
    fn ex24_tile_new() {
        assert_eq!(TilePosition::new(), TilePosition { column: 0, row: 0 })
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day24::solve_default_file().unwrap(), 10)
    }
}
