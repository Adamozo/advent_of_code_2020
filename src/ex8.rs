use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::{self};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_instruction_num<P>(path: &P, num: usize) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    for (line_num, line) in (read_lines(path)?).enumerate() {
        if line_num == num {
            let line = line?;
            return Ok(line);
        }
    }

    Err(anyhow::anyhow!("{}", OperationError::NoOperation))
}

fn load_instructions<P>(path: &P) -> anyhow::Result<HashMap<usize, Operation>>
where
    P: AsRef<Path>,
{
    let mut operations: HashMap<usize, Operation> = HashMap::new();

    for (line_num, line) in (read_lines(path)?).enumerate() {
        let line = line?;
        operations.insert(line_num, Operation::from_str(&line)?);
    }

    Ok(operations)
}

fn evaluate2<P>(path: &P) -> anyhow::Result<i16>
where
    P: AsRef<Path>,
{
    let mut accumulator: i16 = 0;
    let mut visited: Vec<usize> = Vec::new();

    let mut operation_num: usize = 0;

    while !visited.contains(&operation_num) {
        let op: Operation = Operation::from_str(load_instruction_num(&path, operation_num)?.as_str())?;
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

            _ => unreachable!(), // czy da się jakoś to ominąć, ponieważ nie wysąpi inny przypadek
        }
    }

    Ok(accumulator)
}

fn evaluate1<P>(path: &P) -> anyhow::Result<i16>
where
    P: AsRef<Path>,
{
    let operations = load_instructions(path)?;
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

            _ => unreachable!(), // czy da się jakoś to ominąć, ponieważ nie wysąpi inny przypadek
        }
    }

    Ok(accumulator)
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    println!("evaluate1 accumulator: {}", evaluate1(&path)?);
    println!("evaluate2 accumulator: {}", evaluate2(&path)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
    fn test_ex8_evaluate1() {
        assert_eq!(evaluate1(&"data_files/ex8.txt").unwrap(), 5);
        assert!(evaluate1(&"aaa").is_err())
    }

    #[test]
    fn test_ex8_evaluate2() {
        assert_eq!(evaluate2(&"data_files/ex8.txt").unwrap(), 5);
        assert!(evaluate2(&"aaa").is_err())
    }

    #[test]
    fn test_ex8_run_no_file() {
        assert!(run("aaa").is_err())
    }

    #[test]
    fn test_ex8_load_instruction_num_no_file() {
        assert!(load_instruction_num(&"aaa", 1).is_err())
    }

    #[test]
    fn test_ex8_load_instructions_no_file() {
        assert!(load_instructions(&"aaa").is_err())
    }
}
