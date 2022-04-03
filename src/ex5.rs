/*

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

    row * 8 + column = seat id
*/

use std::io::{self};

fn extract_seat_num(input: &str) -> (u32, u32, u32){
    let mut column = 0;
    let base: u32 = 2;
    let mut row = 0;
    for (index,p) in input.chars().enumerate(){
        if p == 'B'{
            row += base.pow(6-index as u32);
        } 

        else if p == 'R'{
            column += base.pow(9-index as u32);
        }
    }

    (column, row, row*8 + column)
}

pub fn run() -> io::Result<()>{

    println!("FBFBBFFRLR: {:?}", extract_seat_num("FBFBBFFRLR"));
    println!("BFFFBBFRRR: {:?}", extract_seat_num("BFFFBBFRRR"));
    println!("FFFBBBFRRR: {:?}", extract_seat_num("FFFBBBFRRR"));
    println!("BBFFBBFRLL: {:?}", extract_seat_num("BBFFBBFRLL"));
    Ok(())
}