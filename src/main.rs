mod ex2;
//mod ex1;
use std::process;

fn main() {
    if let Err(err) = ex2::run("data_files/ex2_passwords.txt".to_string()) {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
