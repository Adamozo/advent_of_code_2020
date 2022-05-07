use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct ShipDirection {
    x: f32,
    y: f32,
}

impl ShipDirection {
    fn rotate(&mut self, angle: f32) {
        let (sin, cos) = angle.to_radians().sin_cos();
        let new_x = (self.x * cos - self.y * sin) * 100_f32;
        let new_y = (self.y * cos + self.x * sin) * 100_f32;

        self.x = new_x.round() / 100_f32;
        self.y = new_y.round() / 100_f32;
    }

    fn count_movement(&self, units: f32) -> (f32, f32) {
        (self.x * units, self.y * units)
    }
}

enum Move {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = &s[0..1];
        let value = &s[1..].parse::<i32>()?;

        match action {
            "N" => Ok(Move::N(*value)),
            "S" => Ok(Move::S(*value)),
            "W" => Ok(Move::W(*value)),
            "E" => Ok(Move::E(*value)),
            "L" => Ok(Move::L(*value)),
            "R" => Ok(Move::R(*value)),
            "F" => Ok(Move::F(*value)),
            o => Err(anyhow::anyhow!("unknown operation {}", o)),
        }
    }
}

fn count_travel_distance(data: &str) -> anyhow::Result<f32> {
    let res: Vec<Move> = data
        .lines()
        .filter_map(|line| line.parse::<Move>().ok())
        .collect();

    let start_direction = ShipDirection { x: 1_f32, y: 0_f32 };

    let final_position = res.iter().fold(
        (0_f32, 0_f32, start_direction),
        |(x, y, mut direction), action| match action {
            Move::N(v) => (x, y + *v as f32, direction),
            Move::S(v) => (x, y - *v as f32, direction),
            Move::W(v) => (x - *v as f32, y, direction),
            Move::E(v) => (x + *v as f32, y, direction),
            Move::L(v) => {
                direction.rotate(*v as f32);
                (x, y, direction)
            },
            Move::R(v) => {
                direction.rotate(-*v as f32);
                (x, y, direction)
            },
            Move::F(v) => {
                let (x_move, y_move) = direction.count_movement(*v as f32);
                (x + x_move, y + y_move, direction)
            },
        },
    );

    Ok(final_position.0.abs() + final_position.1.abs())
}

fn get_data<P>(path: P) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data = get_data(path)?;
    println!("movement: {:?}", count_travel_distance(data.as_str())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex12_count_travel_distance() {
        let data = get_data("data_files/ex12.txt").unwrap();
        assert_eq!(count_travel_distance(data.as_str()).unwrap(), 25.0);

        let data = get_data("data_files/ex12_mydata.txt").unwrap();
        assert_eq!(count_travel_distance(data.as_str()).unwrap(), 41.84);

        let data = get_data("data_files/ex12_invalid.txt").unwrap();
        assert_eq!(count_travel_distance(data.as_str()).unwrap(), 18.0);
    }

    #[test]
    fn test_ex12_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex12_run_file_exists() {
        assert!(!run("data_files/ex12.txt").is_err())
    }
}
