use anyhow::*;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use core::str::FromStr;

pub struct Day17;

impl DaySolver for Day17 {
    type Output = u32;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_17", "data_files/ex17.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let start_grid = _s.parse::<Grid3D>()?;

        let (_, active_cubes) = (1..=6).fold((start_grid, 0u32), |(grid, _), _| {
            let mut active_cubes = 0;
            let mut new_grid = Grid3D::new(grid.columns + 2, grid.rows + 2, grid.depth + 2);

            for depth in 0..new_grid.depth {
                for row in 0..new_grid.rows {
                    for column in 0..new_grid.columns {
                        new_grid.set_cube_state(depth, row, column, &grid, &mut active_cubes);
                    }
                }
            }
            (new_grid, active_cubes)
        });

        Ok(active_cubes)
    }
}

type GridBody = Vec<Vec<Vec<CubeState>>>;
type Grid2DBody = Vec<Vec<CubeState>>;

#[derive(Debug, PartialEq)]
struct Grid3D {
    body:    GridBody,
    columns: usize,
    rows:    usize,
    depth:   usize,
}

impl FromStr for Grid3D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base_body: Grid2DBody = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|field| CubeState::try_from(field).unwrap())
                    .collect()
            })
            .collect();

        let columns = base_body.len();
        let rows = base_body[0].len();
        let depth = 1;

        Ok(Grid3D {
            body: vec![base_body],
            columns,
            rows,
            depth,
        })
    }
}

impl Grid3D {
    fn new(columns: usize, rows: usize, depth: usize) -> Self {
        Grid3D {
            body: vec![vec![vec![CubeState::Inactive; columns]; rows]; depth],
            columns,
            rows,
            depth,
        }
    }

    fn set_cube_state(
        &mut self,
        depth: usize,
        row: usize,
        column: usize,
        previous_grid: &Grid3D,
        active_cubes: &mut u32,
    ) {
        use super::ex17::CubeState::*;

        let adjacent_active_cubes = previous_grid.count_active_adjacent_cubes(
            (row as i32) - 1,
            (column as i32) - 1,
            (depth as i32) - 1,
        );

        if depth == 0
            || row == 0
            || column == 0
            || depth == self.depth - 1
            || row == self.rows - 1
            || column == self.columns - 1
        {
            if adjacent_active_cubes == 3 {
                self.body[depth][row][column] = Active;
                *active_cubes += 1;
            }
        } else {
            match previous_grid.body[depth - 1][row - 1][column - 1] {
                Inactive if adjacent_active_cubes == 3 => {
                    self.body[depth][row][column] = Active;
                    *active_cubes += 1;
                },
                Active if adjacent_active_cubes == 3 || adjacent_active_cubes == 2 => {
                    self.body[depth][row][column] = Active;
                    *active_cubes += 1;
                },

                _ => {
                    self.body[depth][row][column] = Inactive;
                },
            }
        }
    }

    fn count_active_adjacent_cubes(&self, row: i32, col: i32, depth: i32) -> u8 {
        (1..=26)
            .map(|field| match field {
                1 => (-1, -1, -1),
                2 => (0, -1, -1),
                3 => (1, -1, -1),
                4 => (-1, 0, -1),
                5 => (0, 0, -1),
                6 => (1, 0, -1),
                7 => (-1, 1, -1),
                8 => (0, 1, -1),
                9 => (1, 1, -1),
                10 => (-1, -1, 0),
                11 => (0, -1, 0),
                12 => (1, -1, 0),
                13 => (-1, 0, 0),
                14 => (1, 0, 0),
                15 => (-1, 1, 0),
                16 => (0, 1, 0),
                17 => (1, 1, 0),
                18 => (-1, -1, 1),
                19 => (0, -1, 1),
                20 => (1, -1, 1),
                21 => (-1, 0, 1),
                22 => (0, 0, 1),
                23 => (1, 0, 1),
                24 => (-1, 1, 1),
                25 => (0, 1, 1),
                26 => (1, 1, 1),
                _ => unreachable!(),
            })
            .filter(|(x, y, z)| {
                (0..self.columns as i32).contains(&(x + col))
                    && (0..self.rows as i32).contains(&(y + row))
                    && (0..self.depth as i32).contains(&(z + depth))
                    && self.body[(depth + z) as usize][(row + y) as usize][(col + x) as usize]
                        == CubeState::Active
            })
            .count() as u8
    }
}

#[derive(PartialEq, Debug, Clone)]
enum CubeState {
    Active,
    Inactive,
}

impl TryFrom<char> for CubeState {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == '#' {
            Ok(CubeState::Active)
        } else if value == '.' {
            Ok(CubeState::Inactive)
        } else {
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
        assert_eq!(Day17::solve_default_file().unwrap(), 112);
    }

    #[test]
    fn ex17_grid3d_from_str() {
        use super::CubeState::*;
        let input = ".#.\n..#\n###";

        let result = input.parse::<Grid3D>().unwrap();

        let body = vec![vec![
            vec![Inactive, Active, Inactive],
            vec![Inactive, Inactive, Active],
            vec![Active, Active, Active],
        ]];

        assert_eq!(result.body, body);
        assert_eq!(result.columns, 3);
        assert_eq!(result.rows, 3);
        assert_eq!(result.depth, 1);
    }

    #[test]
    fn ex17_cube_state_from() {
        assert_eq!(CubeState::try_from('#').unwrap(), CubeState::Active);
        assert_eq!(CubeState::try_from('.').unwrap(), CubeState::Inactive);
        assert!(CubeState::try_from('-').is_err());
    }
}
