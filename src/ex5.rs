/*
    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBF RLL: row 102, column 4, seat ID 820.
    row * 8 + column = seat id
*/
const _MAX_BIT_POS: usize = 9; // "FBFBBFFRLR".len() - 1;


use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day5;

impl DaySolver for Day5 {
    type Output = i32;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_5", "data_files/ex5.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let res = max_seat_id(_s);
        Ok(res)
    }
}

fn _extract_seat_num2(code: &str) -> u32 {
    code.bytes()
        .enumerate()
        .filter(|(_, b)| *b == b'B' || *b == b'R')
        .fold(0u32, |id, (high_bit_pos, _)| {
            id | (1 << (_MAX_BIT_POS - high_bit_pos))
        })
}

fn extract_seat_num(input: &str) -> u16 {
    let base: u16 = 2;
    let cords: (u16, u16) =
        input
            .chars()
            .enumerate()
            .fold((0u16, 0u16), |(column, row), (index, p)| {
                if p == 'B' {
                    (column + base.pow(6 - index as u32), row)
                } else if p == 'R' {
                    (column, row + base.pow(9 - index as u32))
                } else {
                    (column, row)
                }
            });

    cords.0 * 8 + cords.1
}

fn max_seat_id(input: &str) -> i32
{
    let mut max_seat_num: i32 = -1;

    for coded_seat_num in input.lines() {
        let seat_num = i32::from(extract_seat_num(coded_seat_num));
        println!("{}", &seat_num);
        if seat_num > max_seat_num {
            max_seat_num = seat_num;
        }
    }

    max_seat_num
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use aoc_utils::read_to_string;

    #[test_case("BFFFBBFRRR" => 567)]
    #[test_case("FBFBBFFRLR" => 357)]
    #[test_case("FFFBBBFRRR" => 119)]
    #[test_case("BBFFBBFRLL" => 820)]
    fn test_extract_seat_num(s: &str) -> u16 {
        extract_seat_num(s)
    }

    #[test]
    fn test_max_seat_id(){
        let input = read_to_string("data_files/ex5.txt").unwrap();
        let res = max_seat_id(&input);
        assert_eq!(res,820) 
    }
}
