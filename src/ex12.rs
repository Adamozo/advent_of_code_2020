use lazy_regex::{regex, Lazy, Regex};
use std::fs;
use std::io::{self};
use std::ops::ControlFlow::{Break, Continue};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CaptureError {
    #[error("unable to capture action")]
    CaptureFailed,
}

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

    fn count_movement(&self, units: &f32) -> (f32, f32) {
        (self.x * units, self.y * units)
    }
}

fn count_travel_distance(data: &str) -> Result<f32, CaptureError> {
    let re: &Lazy<Regex> = regex!(r"(?P<direction>\w{1})(?P<step>\d+)");

    let res: Vec<&str> = data.lines().collect();

    let start_direction = ShipDirection { x: 1_f32, y: 0_f32 };

    let final_position = res.iter().try_fold(
        (0_f32, 0_f32, start_direction),
        |(x, y, mut direction), action| match re.captures(action) {
            Some(r) => match &r["direction"] {
                "N" => Continue((x, y + &r["step"].parse::<f32>().unwrap(), direction)),
                "S" => Continue((x, y - &r["step"].parse::<f32>().unwrap(), direction)),
                "W" => Continue((x - &r["step"].parse::<f32>().unwrap(), y, direction)),
                "E" => Continue((x + &r["step"].parse::<f32>().unwrap(), y, direction)),
                "L" => {
                    direction.rotate(r["step"].parse::<f32>().unwrap());
                    Continue((x, y, direction))
                },
                "R" => {
                    direction.rotate(-r["step"].parse::<f32>().unwrap());
                    Continue((x, y, direction))
                },
                "F" => {
                    let (x_move, y_move) =
                        direction.count_movement(&r["step"].parse::<f32>().unwrap());
                    Continue((x + x_move, y + y_move, direction))
                },
                _ => Break(CaptureError::CaptureFailed),
            },
            _ => Break(CaptureError::CaptureFailed),
        },
    );

    match final_position {
        Continue(ok) => Ok(ok.0.abs() + ok.1.abs()),
        Break(err) => Err(err),
    }
}

fn get_data<P>(path: P) -> io::Result<String>
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
    println!("movement: {:?}", count_travel_distance(data.as_str()));
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_case::test_case;

//     #[test_case("data_files/ex10.txt" => Ok(198))]
//     #[test_case("data_files/ex10_error.txt" => Err(AdaptersConnectError::ToBigDifference(49, 200)))]
//     fn test_extract_seat_num(s: &str) -> Result<u64, AdaptersConnectError> {
//         let data = get_data(s).unwrap();
//         connect_adapters(data.as_str())
//     }

//     #[test]
//     fn test_ex10_run_no_file() {
//         assert!(run("aaa").is_err())
//     }

//     #[test]
//     fn test_ex10_run_file_exists() {
//         assert!(!run("data_files/ex10.txt").is_err())
//     }
// }
