use std::fmt::Write as _;
use std::str::FromStr;

use anyhow::{anyhow, Result};

pub fn day07(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 07 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 07 B Input result: {:?}", day_b_total);

    Ok(())
}

struct Equation {
    target: i64,
    params: Vec<i64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once(": ") {
            None => Err(anyhow!("Could not split {:?}", s)),
            Some((left, right)) => {
                let target: i64 = left.parse()?;
                let params: Vec<i64> = right
                    .split(' ')
                    .map(|item| {
                        let result = item.parse::<i64>()?;
                        Ok::<i64, anyhow::Error>(result)
                    })
                    .collect::<Result<Vec<i64>>>()?;

                Ok(Self { target, params })
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply_operator(self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
            Self::Concatenate => {
                let mut str = String::new();
                let _ = write!(&mut str, "{}{}", left, right);
                str.parse::<i64>().unwrap()
            }
        }
    }
}

impl Equation {
    fn get_target(&self) -> i64 {
        self.target
    }

    fn attempt_resolve(&self, operator_types: &Vec<Operator>) -> bool {
        let count = self.params.len();
        let op_sequences = self.gen_permutations(count - 1, operator_types);

        for op_sequence in op_sequences {
            let mut current = self.params[0];

            for (i, op) in op_sequence.into_iter().enumerate() {
                let new = self.params[i + 1];
                current = op.apply_operator(current, new)
            }

            if current == self.target {
                return true;
            }
        }

        false
    }

    fn gen_permutations(&self, len: usize, operator_types: &Vec<Operator>) -> Vec<Vec<Operator>> {
        if len == 0 {
            Vec::new()
        } else if len == 1 {
            let mut vec = Vec::new();
            for op_type in operator_types {
                vec.push([op_type.clone()].to_vec());
                vec.push([op_type.clone()].to_vec());
            }
            vec
        } else {
            let current = if len > 1 {
                self.gen_permutations(len - 1, operator_types)
            } else {
                Vec::new()
            };

            let mut new = Vec::new();
            for entry in current {
                for op_type in operator_types {
                    let mut tmp = entry.clone();
                    tmp.push(op_type.clone());
                    new.push(tmp);
                }
            }

            new
        }
    }
}

fn parse_day(input: String) -> Result<Vec<Equation>> {
    input.lines().map(|line| Equation::from_str(line)).collect()
}

fn compute_day_a(input: &Vec<Equation>) -> Result<i64> {
    let mut total = 0;
    for equation in input {
        if equation.attempt_resolve(&[Operator::Add, Operator::Multiply].to_vec()) {
            total += equation.get_target();
        }
    }
    Ok(total)
}

fn compute_day_b(input: &Vec<Equation>) -> Result<i64> {
    let mut total = 0;
    for equation in input {
        if equation
            .attempt_resolve(&[Operator::Add, Operator::Multiply, Operator::Concatenate].to_vec())
        {
            total += equation.get_target();
        }
    }
    Ok(total)
}
