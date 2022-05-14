use std::fs;
use std::io::{self};
use std::path::Path;

pub fn get_data<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

#[derive(PartialEq, Clone, Debug)]
enum LocationState {
    Occupied,
    Empty,
    Gap,
}

fn num_occupied_adjacent_cells(board: &[Vec<LocationState>], row: &i32, col: &i32) -> u8 {
    (1..=8)
        .map(|field| match field {
            1 => (-1, -1),
            2 => (0, -1),
            3 => (1, -1),
            4 => (-1, 0),
            5 => (1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            8 => (1, 1),
            _ => unreachable!(),
        })
        .filter(|(r, c)| {
            (0..10).contains(&(*r + row))
                && (0..10).contains(&(*c + col))
                && board[(row + r) as usize][(col + c) as usize] == LocationState::Occupied
        })
        .fold(0, |acc, _x| acc + 1)
}

fn update_board2(base_board: &[Vec<LocationState>]) -> (bool, usize, Vec<Vec<LocationState>>) {
    use LocationState::*;

    let mut occupied_seats_num: usize = 0;
    let mut was_any_seat_changed = false;
    let last_row = base_board.len() - 1;
    let last_col = base_board[0].len() - 1;

    let mut result_board: Vec<Vec<LocationState>> = base_board.to_vec();

    for row in 0..=last_row {
        for col in 0..=last_col {
            let occupied = num_occupied_adjacent_cells(base_board, &(row as i32), &(col as i32));
            match base_board[row][col] {
                Empty => {
                    if occupied == 0 {
                        was_any_seat_changed = true;
                        result_board[row][col] = Occupied;

                        occupied_seats_num += 1;
                    } else {
                        result_board[row][col] = Empty;
                    }
                },
                Occupied => {
                    if occupied >= 4 {
                        was_any_seat_changed = true;
                        result_board[row][col] = Empty;
                    } else {
                        result_board[row][col] = Occupied;
                        occupied_seats_num += 1;
                    }
                },
                Gap => continue,
            }
        }
    }

    (was_any_seat_changed, occupied_seats_num, result_board)
}

pub fn count_occupied_seats2(data: &str) -> usize {
    use LocationState::*;

    let mut board1: Vec<Vec<LocationState>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|f| match f {
                    'L' => Empty,
                    '#' => Occupied,
                    '.' => Gap,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    loop {
        let (check_next, res, new_board) = update_board2(&board1);

        if !check_next {
            return res;
        }

        board1 = new_board;
    }
}

fn update_board(
    base_board: &[Vec<LocationState>],
    result_board: &mut [Vec<LocationState>],
) -> (bool, usize) {
    use LocationState::*;

    let mut occupied_seats_num: usize = 0;
    let mut was_any_seat_changed = false;
    let last_row = base_board.len() - 1;
    let last_col = base_board[0].len() - 1;

    for row in 0..=last_row {
        for col in 0..=last_col {
            let occupied = num_occupied_adjacent_cells(base_board, &(row as i32), &(col as i32));
            match base_board[row][col] {
                Empty => {
                    if occupied == 0 {
                        was_any_seat_changed = true;
                        result_board[row][col] = Occupied;

                        occupied_seats_num += 1;
                    } else {
                        result_board[row][col] = Empty;
                    }
                },
                Occupied => {
                    if occupied >= 4 {
                        was_any_seat_changed = true;
                        result_board[row][col] = Empty;
                    } else {
                        result_board[row][col] = Occupied;
                        occupied_seats_num += 1;
                    }
                },
                Gap => continue,
            }
        }
    }

    (was_any_seat_changed, occupied_seats_num)
}

pub fn count_occupied_seats(data: &str) -> usize {
    use LocationState::*;

    let mut board1: Vec<Vec<LocationState>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|f| match f {
                    'L' => Empty,
                    '#' => Occupied,
                    '.' => Gap,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut board2 = board1.clone();

    let mut change_first = false;

    loop {
        let (check_next, res) = {
            if change_first {
                change_first = false;
                update_board(&board2, &mut board1)
            } else {
                change_first = true;
                update_board(&board1, &mut board2)
            }
        };

        if !check_next {
            return res;
        }
    }
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data = get_data(path)?;

    println!("Version1");
    println!(
        "There are {} occupied seats",
        count_occupied_seats(data.as_str())
    );

    println!("Version2");
    println!(
        "There are {} occupied seats",
        count_occupied_seats2(data.as_str())
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("data_files/ex11.txt" => (37, 37))]
    fn test_ex11_counter_methods(s: &str) -> (usize, usize) {
        let data = get_data(s).unwrap();
        (count_occupied_seats(data.as_str()), count_occupied_seats2(data.as_str()))
    }

    #[test]
    fn test_ex11_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex11_run_file_exists() {
        assert!(!run("data_files/ex11.txt").is_err())
    }
}
