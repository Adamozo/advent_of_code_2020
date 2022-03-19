use lazy_regex::{regex, Lazy, Regex};
use std::str::FromStr;
use test_case::test_case;

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
// pub fn run() -> Result<Password, String>{
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
