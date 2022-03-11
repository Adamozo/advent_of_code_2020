use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

pub struct Password{
    min_number: u16,
    max_number: u16,
    checked_char: char,
    passwd: String,
}

impl Password{
    pub fn is_valid(&self) -> bool {
        let counter: u16 = self.passwd.chars().into_iter().filter(|c| c == &self.checked_char).count().try_into().unwrap();
    
        (self.min_number .. self.max_number+1).contains(&counter)
    } 

}

impl FromStr for Password{
    type Err = ParseIntError;
    // 1-3 a: abcde
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let val = Regex::new(r"(\d+)-(\d+) (\w{1}): (\w+)").unwrap();

        match val.is_match(s)  {
            true => {
                let coords: Vec<&str> = s.split(' ').collect();
                let ranges: Vec<&str> = coords[0].split("-").collect();
                        
                let min_number = ranges[0].parse::<u16>()?;
                let max_number = ranges[1].parse::<u16>()?;
                let temp: Vec<char> = coords[1].chars().collect();

                Ok(Password { min_number: min_number, max_number: max_number, checked_char: temp[0],  passwd: coords[2].to_string()})
            }

            false => {
                Ok(Password {min_number: s.parse::<u16>().unwrap(), max_number: s.parse::<u16>().unwrap(), checked_char: 'e', passwd: "error".to_string()})
            }
        }
        
        
    }
}

pub fn run() {
    let p1 = Password::from_str("1-3 a: abcde").unwrap();
    assert_eq!(true, Password::is_valid(&p1));

    let p2 = Password::from_str("2-3 a: abcde").unwrap();
    assert_eq!(false, Password::is_valid(&p2));
}
