use aoc_utils::{DayInfo, DaySolver};
use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;

// -----------------------------------------------------------------------------
pub struct Day17other;

impl DaySolver for Day17other {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_17", "data_files/ex17.txt");

    fn solution(s: &str) -> anyhow::Result<Self::Output> {
        let mut grid = Grid::initial(s);

        grid.cycle_n_times(6);

        Ok(grid.count_active_cells())
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Grid {
    data: HashSet<Point3D>,
}

// -----------------------------------------------------------------------------

impl Point3D {
    #[inline]
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

// -----------------------------------------------------------------------------

impl Grid {
    fn initial(s: &str) -> Self {
        fn is_active(b: u8) -> bool {
            b == b'#'
        }

        let mut data: HashSet<Point3D> = HashSet::default();

        for (row, cells_states_as_bytes) in s.lines().enumerate() {
            for (col, byte) in cells_states_as_bytes.bytes().enumerate() {
                if is_active(byte) {
                    let point = Point3D::new(col as i32, row as i32, 0);
                    data.insert(point);
                }
            }
        }

        Self { data }
    }

    fn cycle_n_times(&mut self, n: u32) {
        let mut env_point2count_active: HashMap<Point3D, u32> = HashMap::default();

        for _ in 1..=n {
            self.cycle(&mut env_point2count_active);
        }
    }

    fn cycle(&mut self, env_point2count_active: &mut HashMap<Point3D, u32>) {
        for point in self.data.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        *env_point2count_active
                            .entry(Point3D::new(point.x + dx, point.y + dy, point.z + dz))
                            .or_insert(0) += 1;
                    }
                }
            }
        }

        for (env_point, count_active) in env_point2count_active.drain() {
            if count_active == 3 && !self.data.contains(&env_point) {
                self.data.insert(env_point);
            } else if !(2..=3).contains(&(count_active - 1)) && self.data.contains(&env_point) {
                self.data.remove(&env_point);
            }
        }
    }

    fn count_active_cells(&self) -> u32 {
        self.data.len() as u32
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day17other::solve_default_file().unwrap(), 112)
    }
}