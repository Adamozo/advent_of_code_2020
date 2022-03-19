use lazy_regex::{regex, Lazy, Regex};
use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Password {
    min_number: u16,
    max_number: u16,
    checked_char: char,
    passwd: String,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        let counter: u16 = self
            .passwd
            .chars()
            .filter(|c| *c == self.checked_char)
            .count() as u16;

        (self.min_number..=self.max_number).contains(&counter)
    }
}

impl FromStr for Password {
    type Err = String;
    // 1-3 a: abcde
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: &Lazy<Regex> = regex!(r"(?P<min>\d+)-(?P<max>\d+) (?P<checked_char>\w{1}): (?P<passwd>\w+)");
        let caps = re.captures(s);

        if let Some(r) = caps {
            Ok(Password {
                min_number: r[1].parse::<u16>().unwrap(),
                max_number: r[2].parse::<u16>().unwrap(),
                checked_char: r[3].chars().next().unwrap(),
                passwd: r[4].to_string(),
            })
        } else {
            Err("unable to capture".to_string())
        }
        

    }
}

pub fn run(){
     let p1  = "1-3 a: abcde".parse::<Password>().unwrap();
     match p1 {
        r => {
                         println!(
                 "{:?} {:?} {:?} {:?}",
                                  r.min_number, r.max_number, r.checked_char, r.passwd
             );
             println!("is_valid: {:?}", true);
         }
        _ => println!(" error")
     }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'a', passwd: "abcde".to_string()}); "valid webiste 1")]
    #[test_case("1-3 b: cdefg" => Ok(Password{min_number: 1, max_number: 3, checked_char: 'b', passwd: "cdefg".to_string()}); "valid website 2")]
    #[test_case("2-9 c: ccccccccc" => Ok(Password{min_number: 2, max_number: 9, checked_char: 'c', passwd: "ccccccccc".to_string()}); "valid website 3")]
    #[test_case("1-c a: abcde" => Err("unable to capture".to_string()); "invalid letter as max_number")]
    #[test_case("c-3 a: abcde" => Err("unable to capture".to_string()); "invalid letter as min_number")]
    #[test_case("1-3 1: abcde" => Ok(Password{min_number: 1, max_number: 3, checked_char: '1', passwd: "abcde".to_string()}); "valid num as checked char")]
    #[test_case("1-3 a: " => Err("unable to capture".to_string()); "invalid lack of password")]
    fn test_from_str(s: &str) -> Result<Password, String>{
        Password::from_str(s)
    }


    #[test_case("1-3 a: abcde" => true ; "valid 1-3 a: abcde")]
    #[test_case("1-3 b: cdefg" =>  false  ; "invalid 1-3 b: cdefg")]
    #[test_case("2-9 c: ccccccccc" =>  true  ; "valid 2-9 c: ccccccccc")]
    #[test_case("3-1 a: abcde" => false ; "invalid 3-1 a: abcde")]
    fn test_is_valid(s: &str) -> bool{
        let p1 = Password::from_str(s).unwrap();
        Password::is_valid(&p1)
    }
}
