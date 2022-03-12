//mod ex1;
mod ex2;

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::ex2::Password;
    use std::str::FromStr;

    #[test_case("1-3 a: abcde", true ; "valid 1-3 a: abcde")]
    #[test_case("1-3 b: cdefg",  false  ; "invalid 1-3 b: cdefg")]
    #[test_case("2-9 c: ccccccccc",  true  ; "valid 2-9 c: ccccccccc")]
    #[test_case("3-1 a: abcde", false ; "valid 3-1 a: abcde")]
    fn is_valid_test(s: &str, res: bool){
        let p1 = Password::from_str(s).unwrap();
        assert_eq!(res, Password::is_valid(&p1));
    }
}

fn main() {
    ex2::run();
}
