use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use lazy_regex::{regex, Lazy, Regex};
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day7;

impl DaySolver for Day7 {
    type Output = usize;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_7", "data_files/ex7.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let rules = _s.parse::<Rules>()?;
        let res = rules
            .get_bags()
            .filter(|bag| rules.contains_shiny_gold(bag))
            .count();

        Ok(res)
    }
}

struct Rules {
    body: HashMap<String, VecDeque<String>>,
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: &Lazy<Regex> = regex!(r"(\d+) (?P<color>\w+ \w+) bags?");

        let body: HashMap<String, VecDeque<String>> = s
            .lines()
            .map(|line| {
                let (bag, contained_bags) = line.split_once(" bags contain ").unwrap();

                let mut value: VecDeque<String> = VecDeque::new();

                for i in contained_bags.split(", ") {
                    match re.captures(i) {
                        None => {
                            break;
                        },
                        Some(cap) => {
                            value.push_back(cap["color"].to_string());
                        },
                    }
                }

                (bag.to_owned(), value)
            })
            .collect();

        Ok(Rules { body })
    }
}

impl Rules {
    fn contains_shiny_gold(&self, bag: &String) -> bool {
        let mut queue = self.body.get(bag).unwrap().to_owned();

        while !queue.is_empty() {
            if let Some(checked) = queue.pop_front() {
                if checked == "shiny gold" {
                    return true;
                }

                queue.append(&mut self.body.get(&checked).unwrap().to_owned());
            }
        }

        false
    }

    fn get_bags(&self) -> std::collections::hash_map::Keys<'_, String, VecDeque<String>> {
        self.body.keys()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test_case::test_case;

    use super::*;

    #[test_case("dotted black" => false)]
    #[test_case("vibrant plum" => false)]
    #[test_case("bright white" => true)]
    #[test_case("light red" => true)]
    fn ex7_from_str(bag: &str) -> bool {
        let input = read_to_string("data_files/ex7.txt").unwrap();

        input
            .parse::<Rules>()
            .unwrap()
            .contains_shiny_gold(&bag.to_string())
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day7::solve_default_file().unwrap(), 4)
    }
}
