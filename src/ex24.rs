use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day24;

#[derive(PartialEq, Debug)]
enum TileSide {
    White,
    Black,
}

#[derive(Debug)]
struct Tile {
    column: i32,
    row: i32,
    color: TileSide,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.column == other.column && self.row == other.row
    }
}

impl Tile {
    fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            color: TileSide::White,
        }
    }

    fn rotate(&mut self) {
        if self.color == TileSide::White {
            self.color = TileSide::Black;
        } else {
            self.color = TileSide::White;
        }
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

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (black_tiles_counter, _) = _s.lines().map(get_tile).fold(
            (0, Vec::new()),
            |(black_tile_counter, mut tiles): (u32, Vec<Tile>), mut tile| {
                if let Some(index) = tiles.iter().position(|element| *element == tile) {
                    tiles[index].rotate();

                    if tiles[index].color == TileSide::White {
                        return (black_tile_counter - 1, tiles);
                    }
                } else {
                    tile.rotate();
                    tiles.push(tile);
                }

                (black_tile_counter + 1, tiles)
            },
        );

        Ok(black_tiles_counter)
    }
}

fn get_tile(line: &str) -> Tile {
    let (_, tile) = line
        .chars()
        .fold((None, Tile::new()), |(acc_letter, mut tile), new_letter| {
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
        });

    tile
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sesenwnenenewseeswwswswwnenewsewsw" => Tile{column: -3, row: 2, color: TileSide::White})]
    #[test_case("neeenesenwnwwswnenewnwwsewnenwseswesw" => Tile{column: 1, row: -3, color: TileSide::White})]
    #[test_case("seswneswswsenwwnwse" => Tile{column: -3, row: 3, color: TileSide::White})]
    fn ex24_get_tile(line: &str) -> Tile {
        get_tile(line)
    }

    #[test_case(&['s', 'e'] => Tile{column: 0, row: 1, color: TileSide::White})]
    #[test_case(&['n', 'e'] => Tile{column: 1, row: -1, color: TileSide::White})]
    #[test_case(&['e'] => Tile{column: 1, row: 0, color: TileSide::White})]
    #[test_case(&['s', 'w'] => Tile{column: -1, row: 1, color: TileSide::White})]
    #[test_case(&['n', 'w'] => Tile{column: 0, row: -1, color: TileSide::White})]
    #[test_case(&['w'] => Tile{column: -1, row: 0, color: TileSide::White})]
    fn ex24_tile_shift(direction: &[char]) -> Tile {
        let mut tile = Tile::new();
        tile.shift(direction);
        tile
    }

    #[test]
    fn ex24_tile_rotate() {
        use TileSide::*;
        let mut tile = Tile::new();
        assert_eq!(tile.color, White);
        tile.rotate();
        assert_eq!(tile.color, Black);
        tile.rotate();
        assert_eq!(tile.color, White)
    }

    #[test]
    fn ex24_tile_new() {
        assert_eq!(
            Tile::new(),
            Tile {
                column: 0,
                row: 0,
                color: TileSide::White
            }
        )
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day24::solve_default_file().unwrap(), 10)
    }
}
