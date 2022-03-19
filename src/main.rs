mod ex2;
//mod ex1;
use std::process;



fn main() {
    if let Err(err) = ex2::run() {
        eprintln!("Error: {:?}", err);
        //process::exit(1);
    }
}


