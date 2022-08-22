use std::collections::HashSet;

use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day6;

impl DaySolver for Day6 {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_6", "data_files/ex6.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let res = count_answers2(_s)?;
        Ok(res)
    }
}

fn mapp_char(c: char) -> u16 {
    match c {
        'a' => 2,
        'b' => 3,
        'c' => 5,
        'd' => 7,
        'e' => 11,
        'f' => 13,
        'g' => 17,
        'h' => 19,
        'i' => 23,
        'j' => 29,
        'k' => 31,
        'l' => 37,
        'm' => 41,
        'n' => 43,
        'o' => 47,
        'p' => 53,
        'q' => 59,
        'r' => 61,
        's' => 67,
        't' => 71,
        'u' => 73,
        'v' => 79,
        'w' => 83,
        'x' => 89,
        'y' => 97,
        'z' => 101,
        _ => unreachable!(),
    }
}

pub fn count_answers4(pattern: &str) -> u32 {
    pattern
        .split("\n\n")
        .map(|group| {
            group
                .bytes()
                .filter(|e| *e != b'\n')
                .fold(0u32, |answers, b| answers | 1 << (b - b'a'))
                .count_ones()
        })
        .sum::<u32>()
}

pub fn count_answers3_bench(pattern: &str) -> usize {
    let mut mapper = 1;
    let mut counter: usize = 0;
    let mut questions: usize = 0;

    for line in pattern.split('\n').map(|l| l.trim()) {
        let line = line;
        if line.is_empty() {
            counter += questions;
            mapper = 1;
            questions = 0;
        } else {
            for question in line.chars() {
                let to_check = mapp_char(question) as u128;
                if mapper % to_check != 0 {
                    mapper *= to_check;
                    questions += 1;
                }
            }
        }
    }

    counter += questions;

    counter
}

pub fn count_answers3(input: &str) -> anyhow::Result<usize>
{
    let mut mapper = 1;
    let mut counter: usize = 0;
    let mut questions: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            counter += questions;
            mapper = 1;
            questions = 0;
        } else {
            for question in line.chars() {
                let to_check = mapp_char(question) as u128;
                if mapper % to_check != 0 {
                    mapper *= to_check;
                    questions += 1;
                }
            }
        }
    }

    counter += questions;

    Ok(counter)
}

pub fn count_answers2_bench(pattern: &str) -> usize {
    let mut questions: Vec<char> = Vec::new();
    let mut counter: usize = 0;

    for line in pattern.split('\n').map(|l| l.trim()) {
        let line = line;
        if line.is_empty() {
            questions.sort_unstable();
            questions.dedup();
            counter += questions.len();
            questions = Vec::new();
        } else {
            for question in line.chars() {
                questions.push(question);
            }
        }
    }

    questions.sort_unstable();
    questions.dedup();
    counter += questions.len();

    counter
}

pub fn count_answers2(input: &str) -> anyhow::Result<usize>
{
    let mut questions: Vec<char> = Vec::new();
    let mut counter: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            questions.sort_unstable();
            questions.dedup();
            counter += questions.len();
            questions = Vec::new();
        } else {
            for question in line.chars() {
                questions.push(question);
            }
        }
    }

    questions.sort_unstable();
    questions.dedup();
    counter += questions.len();

    Ok(counter)
}

pub fn count_answers_bench(pattern: &str) -> usize {
    let mut questions: HashSet<char> = HashSet::new();
    let mut counter: usize = 0;

    for line in pattern.split('\n').map(|l| l.trim()) {
        let line = line;
        if line.is_empty() {
            counter += questions.len();
            questions = HashSet::new();
        } else {
            for question in line.chars() {
                questions.insert(question);
            }
        }
    }

    counter += questions.len();

    counter
}

pub fn count_answers(input: &str) -> anyhow::Result<usize>
{
    let mut questions: HashSet<char> = HashSet::new();
    let mut counter: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            counter += questions.len();
            questions = HashSet::new();
        } else {
            for question in line.chars() {
                questions.insert(question);
            }
        }
    }

    counter += questions.len();

    Ok(counter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::read_to_string;

    #[test]
    fn test_count_answers() {
        let input = read_to_string("data_files/ex6.txt").unwrap();
        assert_eq!(count_answers(&input).unwrap(), 11)
    }
}
