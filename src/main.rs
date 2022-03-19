//mod ex1;
mod ex1;
// use std::process;

// #[cfg(test)]
// mod tests {
//     use crate::ex2::Password;
//     use std::str::FromStr;
//     use test_case::test_case;

//     #[test_case("1-3 a: abcde", true ; "valid 1-3 a: abcde")]
//     #[test_case("1-3 b: cdefg",  false  ; "invalid 1-3 b: cdefg")]
//     #[test_case("2-9 c: ccccccccc",  true  ; "valid 2-9 c: ccccccccc")]
//     #[test_case("3-1 a: abcde", false ; "valid 3-1 a: abcde")]
//     fn is_valid_test(s: &str, res: bool) {
//         let p1 = Password::from_str(s).unwrap();
//         assert_eq!(res, Password::is_valid(&p1));
//     }
// }

fn main() {
    // if let Err(err) = ex2::run() {
    //     eprintln!("Error: {:?}", err);
    //     process::exit(1);
    // }

    //ex1
     let nums: [u32; 6] = [1721, 979, 366, 299, 675, 1456];
     let sum: u32 = 2020;

     let res: Vec<(u32, u32)> = ex1::check_sum(&nums, &sum);
     println!("Nums: {:?}", &nums);
     println!("Expected sum: {:?}", &sum);
     println!("Result: {:?}", &res);

    
}
