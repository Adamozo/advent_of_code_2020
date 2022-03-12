use std::str::FromStr;
use std::num::ParseIntError;
use std::string::ParseError;
use lazy_regex::regex;

pub struct Password{
    min_number: u16,
    max_number: u16,
    checked_char: char,
    passwd: String,
}

impl Password{
    pub fn is_valid(&self) -> bool {
        let counter: u16 = self.passwd.chars().into_iter().filter(|c| c == &self.checked_char).count().try_into().unwrap();
    
        (self.min_number..=self.max_number).contains(&counter)
    } 

}

enum PasswordError{
    ParseIntError,
    ParseError,
    Infallible
}


impl FromStr for Password{
    type Err = ParseIntError;
    // 1-3 a: abcde
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\d+)-(\d+) (\w{1}): (\w+)");
        let caps = re.captures(s);

        match caps {
            Some(r) => {        
                let min = r.get(1).map_or("", |m| m.as_str()).parse::<u16>()?;
                let max = r.get(2).map_or("", |m| m.as_str()).parse::<u16>()?;
                let c = match r.get(3).map_or("", |m| m.as_str()).chars().next() {
                    Some(r) => r,
                    None => panic!()
                };

                let password = r.get(4).map_or("", |m| m.as_str());

                Ok(Password { min_number: min, max_number: max, checked_char: c,  passwd: password.to_string()})
            },
            None => panic!()
        }        
    }
}

pub fn run() {
    let p1 = Password::from_str("1--3 a: abcde");
    match p1 {
        Ok(r) => println!("{:?} {:?} {:?} {:?}", r.min_number, r.max_number, r.checked_char, r.passwd),
        Err(r) => println!(" error {:?}", r),
        _ => println!("missmatch")
    }       
}
