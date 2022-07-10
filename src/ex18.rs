use anyhow::Ok;
use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use std::str::FromStr;

pub struct Day18;

impl DaySolver for Day18 {
    type Output = u32;

    const INFO: DayInfo = DayInfo::with_day_and_file_and_variant(
        "day_18",
        "data_files/ex18.txt",
        "vector with tuple",
    );

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let expresions = _s.replace('(', "( ").replace(')', " )");

        let i: u32 = expresions
            .lines()
            .map(|line| line.parse::<Evaluator>())
            .filter(|evaluator| (*evaluator).is_ok())
            .map(|evaluator| evaluator.unwrap().evaluate())
            .sum();

        Ok(i)
    }
}

#[derive(Debug, PartialEq)]
enum Component {
    Num(u32),
    Multiply,
    Sum,
    LeftBracket,
}

fn is_operator(element: &str) -> bool {
    element == "*" || element == "+"
}

fn get_operator(element: &str) -> Component {
    match element {
        "*" => Component::Multiply,
        "+" => Component::Sum,
        _ => unreachable!(),
    }
}

fn pop_two(vector: &mut Vec<u32>) -> (u32, u32) {
    (vector.pop().unwrap(), vector.pop().unwrap())
}

struct Evaluator {
    postfix: Vec<Component>,
}

impl FromStr for Evaluator {
    type Err = anyhow::Error;

    // basic implementation of infix to postfix converter
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut postfix: Vec<Component> = Vec::new();
        let mut stack: Vec<Component> = Vec::new();

        for ch in s.split(' ').filter(|&element| element.len() > 0) {
            if ch == "(" {
                stack.push(Component::LeftBracket);
            } else if ch == ")" {
                while !stack.is_empty() && stack[stack.len() - 1] != Component::LeftBracket {
                    if let Some(element) = stack.pop() {
                        postfix.push(element);
                    }
                }

                if !stack.is_empty() && stack[stack.len() - 1] == Component::LeftBracket {
                    let _t = stack.pop();
                }
            } else if is_operator(ch) {
                while !stack.is_empty()
                    && (stack[stack.len() - 1] == Component::Multiply
                        || stack[stack.len() - 1] == Component::Sum)
                {
                    if let Some(element) = stack.pop() {
                        postfix.push(element)
                    }
                }

                stack.push(get_operator(ch))
            } else {
                // case when ch is number
                let t = Component::Num((ch).parse::<u32>()?);
                postfix.push(t);
            }
        }

        while !stack.is_empty() {
            if let Some(element) = stack.pop() {
                postfix.push(element);
            }
        }

        Ok(Evaluator { postfix })
    }
}

impl Evaluator {
    fn evaluate(&self) -> u32 {
        use super::ex18::Component::*;

        let res = self
            .postfix
            .iter()
            .fold(Vec::new(), |mut stack, element| match *element {
                Num(number) => {
                    stack.push(number);

                    stack
                },

                Sum => {
                    let (a, b) = pop_two(&mut stack);

                    stack.push(a + b);

                    stack
                },

                Multiply => {
                    let (a, b) = pop_two(&mut stack);

                    stack.push(a * b);

                    stack
                },

                _ => unreachable!(),
            });

        res[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Component::*;
    use super::*;
    use test_case::test_case;

    #[test_case("2 * ( 3 + ( 4 * 5 ) )" => vec![Num(2), Num(3), Num(4), Num(5), Multiply, Sum, Multiply])]
    #[test_case("5 + ( 8 * 3 + 9 + 3 * 4 * 3 )" => vec![Num(5), Num(8), Num(3), Multiply, Num(9), Sum, Num(3), Sum, Num(4), Multiply, Num(3), Multiply, Sum])]
    fn ex18_evaluator_from_str(s: &str) -> Vec<Component> {
        s.parse::<Evaluator>().unwrap().postfix
    }

    #[test_case("2 * ( 3 + ( 4 * 5 ) )" => 46)]
    #[test_case("5 + ( 8 * 3 + 9 + 3 * 4 * 3 )" => 437)]
    #[test_case("( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2" => 13632)]
    fn ex18_evaluate(s: &str) -> u32 {
        s.parse::<Evaluator>().unwrap().evaluate()
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day18::solve_default_file().unwrap(), 26335)
    }
}
