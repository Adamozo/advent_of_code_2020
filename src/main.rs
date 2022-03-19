mod ex2;
//mod ex1;
// use std::process;
use regex::Regex;
use std::str::FromStr;

fn main() {
    // if let Err(err) = ex2::run() {
    //     eprintln!("Error: {:?}", err);
    //     process::exit(1);
    // }
    let rg_w_named = Regex::new(r"was (?P<year>\d+)").unwrap();
    match rg_w_named.captures("The year was 2016") {
        // Named capures groups are accessed via Captures::name
        // Prints Some("2016")
        Some(x) =>  {
            println!("{:?}", u32::from_str(&x["year"]).unwrap());
            let t = (&x["year"]).parse::<u32>().unwrap();
            println!("{:?}", t)
        },
        None    => unreachable!()
    }
}
