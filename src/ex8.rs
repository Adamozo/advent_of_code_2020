use std::fs::File;
use std::io::BufRead;
use std::io::{self};
use std::path::Path;
use thiserror::Error;
use std::str::FromStr;
use std::collections::HashMap;


#[derive(Error, Debug, PartialEq)]
pub enum OperationError {
    #[error("unable to parse instruction")]
    ParseInstructionError,

    #[error("unable to parse step")]
    ParseStepError,
}

#[derive(Debug)]
struct Operation{
    instruction: u8, // nop -> 0, acc -> 1, jmp ->2
    step: i16,
}

impl FromStr for Operation{
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|c| c == '+')
                                 .split(' ')
                                 .collect();

        let instruction = match coords[0] {
            "nop" => 0,
            "acc" => 1,
            "jmp" => 2,
            _ => return Err(OperationError::ParseInstructionError)
        };

        let step = match i16::from_str(coords[1]){
            Ok(p) => p,
            Err(p) => return Err(OperationError::ParseStepError)
        };

        Ok(Operation{instruction, step})
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_instructions1<P>(path: P) -> anyhow::Result<HashMap<usize, Operation>>
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

fn evaluate1<P>(path: P) -> anyhow::Result<i32>
where
    P: AsRef<Path>,
{
    let operations = load_instructions1(path)?;
    let mut accumulator: i32 = 0;
    let mut visited: Vec<usize> = Vec::new();

    let mut operation_num: usize = 0;

    while !visited.contains(&operation_num) {
        visited.push(operation_num);
        let op = &operations[&operation_num];
        match op.instruction{
            0 => {
                operation_num += 1;
            },
            1 => {
                operation_num += 1;
                accumulator += op.step as i32;
            },

            2 => {
                operation_num = (operation_num as i16 + op.step) as usize;
            },

            _ => unreachable!() // czy da się jakoś to ominąć, ponieważ nie wysąpi inny przypadek
        }
    }

    Ok(accumulator)
}

pub fn run<P>(path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    //let res = Operation::from_str("jmp -3").unwrap();
    println!("accumulator: {:?}", evaluate1(path)?);
    Ok(())
}

// tests

// from string z test case
// load data sciezka i wynik
// run sciezka i wynik
// evaluate cale