//mod ex1;
mod ex2;

#[cfg(test)]
mod tests {
    use crate::ex2::Password;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1-3 a: abcde", true ; "when both operands are negative")]
    #[test_case("1-3 b: cdefg",  false  ; "when both operands are positive")]
    #[test_case("2-9 c: ccccccccc",  true  ; "when operands are swapped")]
    fn is_valid_test(s: &str, res: bool) {
        let p1 = Password::from_str(s).unwrap();
        assert_eq!(res, Password::is_valid(&p1));
    }
}

fn main() {
    ex2::run();
}
