/*

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBF RLL: row 102, column 4, seat ID 820.

    row * 8 + column = seat id
*/

use std::io::{self};

fn extract_seat_num(input: &str) -> u16{
    let base: u16 = 2;
    let cords:(u16, u16) = input.chars().enumerate().fold((0u16, 0u16),|(column, row), (index, p)|  {
        if p == 'B'{
            (column + base.pow(6-index as u32), row)
        } 
        else if p == 'R'{
            (column , row + base.pow(9-index as u32))
        }

        else{
            (column, row)
        }
    });


    cords.0*8 + cords.1
}

pub fn run() -> io::Result<()>{

    println!("FBFBBFFRLR: {:?}", extract_seat_num("FBFBBFFRLR"));
    println!("BFFFBBFRRR: {:?}", extract_seat_num("BFFFBBFRRR"));
    println!("FFFBBBFRRR: {:?}", extract_seat_num("FFFBBBFRRR"));
    println!("BBFFBBFRLL: {:?}", extract_seat_num("BBFFBBFRLL"));

    Ok(())
}