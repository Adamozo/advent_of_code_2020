use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

pub struct Day8;

impl DaySolver for Day8 {
    type Output = i16;

    const INFO: DayInfo = DayInfo::with_day_and_file("day_8", "data_files/ex8.txt");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let res = evaluate1(_s)?;
        Ok(res)
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperationError {
    #[error("unable to parse operation")]
    ParseOperationError,

    #[error("unknown operation `{0}`")]
    UnknownOperation(String),

    #[error("unable to parse operation argument")]
    ParseArgumentError,

    #[error("unable to find operation with given number")]
    NoOperation,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Nop,
    Acc(i16),
    Jmp(i16),
}

impl FromStr for Operation {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, argument) = s
            .split_once(' ')
            .ok_or(OperationError::ParseOperationError)?;

        let argument = i16::from_str(argument).map_err(|_| OperationError::ParseArgumentError)?;

        let operation = match operation {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc(argument),
            "jmp" => Operation::Jmp(argument),
            _ => return Err(OperationError::UnknownOperation(operation.into())),
        };

        Ok(operation)
    }
}

fn _load_instruction_num(input: &str, num: usize) -> anyhow::Result<String>
{
    for (line_num, line) in input.lines().enumerate() {
        if line_num == num {
            return Ok(line.to_owned());
        }
    }

    Err(anyhow::anyhow!("{}", OperationError::NoOperation))
}

fn load_instructions(input: &str) -> anyhow::Result<HashMap<usize, Operation>>
{
    let mut operations: HashMap<usize, Operation> = HashMap::new();

    for (line_num, line) in input.lines().enumerate() {
        operations.insert(line_num, Operation::from_str(line)?);
    }

    Ok(operations)
}

fn _evaluate2(input: &str) -> anyhow::Result<i16>
{
    let mut accumulator: i16 = 0;
    let mut visited: Vec<usize> = Vec::new();

    let mut operation_num: usize = 0;

    while !visited.contains(&operation_num) {
        let op: Operation =
            Operation::from_str(_load_instruction_num(input, operation_num)?.as_str())?;
        visited.push(operation_num);

        match op {
            Operation::Nop => {
                operation_num += 1;
            },
            Operation::Acc(value) => {
                operation_num += 1;
                accumulator += value;
            },

            Operation::Jmp(num) => {
                operation_num = (operation_num as i16 + num) as usize;
            },
        }
    }

    Ok(accumulator)
}

fn evaluate1(input: &str) -> anyhow::Result<i16>
{
    let operations = load_instructions(input)?;
    let mut accumulator: i16 = 0;
    let mut visited: Vec<usize> = Vec::new();

    let mut operation_num: usize = 0;

    while !visited.contains(&operation_num) {
        visited.push(operation_num);
        let op = &operations[&operation_num];
        match op {
            Operation::Nop => {
                operation_num += 1;
            },
            Operation::Acc(value) => {
                operation_num += 1;
                accumulator += value;
            },

            Operation::Jmp(num) => {
                operation_num = (operation_num as i16 + num) as usize;
            },
        }
    }

    Ok(accumulator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use aoc_utils::read_to_string;

    #[test_case("nop +0" => Ok(Operation::Nop); "nop +0 ok")]
    #[test_case("acc +1" => Ok(Operation::Acc(1)); "acc +1 ok")]
    #[test_case("jmp +4" => Ok(Operation::Jmp(4)); "jmp +4 ok")]
    #[test_case("jmp -3" => Ok(Operation::Jmp(-3)); "jmp -3 ok")]
    #[test_case("a-3" => Err(OperationError::ParseOperationError); "can't parse input string as an operation")]
    #[test_case("a -3" => Err(OperationError::UnknownOperation("a".into())); "instruction a is not valid")]
    #[test_case("jmp a" => Err(OperationError::ParseArgumentError); "step a is not valid")]
    fn test_ex8_operation_from_str(input: &str) -> Result<Operation, OperationError> {
        input.parse::<Operation>()
    }


    #[test]
    fn test_ex8_load_instruction_num_no_file() {
        assert!(_load_instruction_num(&"aaa", 1).is_err())
    }

    #[test]
    fn test_ex8_load_instructions_no_file() {
        assert!(load_instructions(&"aaa").is_err())
    }
    #[test]
    fn test_ex8_evaluate1() {
        let input = read_to_string("data_files/ex8.txt").unwrap();
        assert_eq!(evaluate1(&input).unwrap(), 5);
    }

    #[test]
    fn test_ex8_evaluate2() {
        let input = read_to_string("data_files/ex8.txt").unwrap();
        assert_eq!(_evaluate2(&input).unwrap(), 5);
    }
}
