use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Default)]
struct ShipDirection {
    x: f32,
    y: f32,
}

impl ShipDirection {
    fn new_xy(x: f32, y: f32) -> Self {
        Self { x, y }
    }

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

#[derive(Debug, Default)]
struct Position {
    x: f32,
    y: f32,
    direction: ShipDirection,
}

impl Position {
    fn new_with_xy_direction(x: f32, y: f32) -> Self {
        Self {
            direction: ShipDirection::new_xy(x, y),
            ..Default::default()
        }
    }

    fn change(&mut self, action: Move) {
        use Move::*;

        let (x_delta, y_delta) = match action {
            North(v) | South(v) => (0_f32, v as f32),
            West(v) | East(v) => (v as f32, 0_f32),
            Left(v) | Right(v) => {
                self.direction.rotate(v as f32);
                (0_f32, 0_f32)
            },
            Forward(v) => self.direction.count_movement(v as f32),
        };

        self.x += x_delta;
        self.y += y_delta;
    }

    fn distance(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }
}

enum Move {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;

        let action = &s[0..1];
        let value = &s[1..].parse::<i32>()?;

        match action {
            "N" => Ok(North(*value)),
            "S" => Ok(South(-*value)),
            "W" => Ok(West(-*value)),
            "E" => Ok(East(*value)),
            "L" => Ok(Left(*value)),
            "R" => Ok(Right(-*value)),
            "F" => Ok(Forward(*value)),
            _ => Err(anyhow::anyhow!("unknown action {}", action)),
        }
    }
}

fn count_travel_distance(data: &str) -> anyhow::Result<f32> {
    let final_position = data
        .lines()
        .filter_map(|line| line.parse::<Move>().ok())
        .fold(
            Position::new_with_xy_direction(1_f32, 0_f32),
            |mut position, action| {
                position.change(action);
                position
            },
        );

    Ok(final_position.distance())
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