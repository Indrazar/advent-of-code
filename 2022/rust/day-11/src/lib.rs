#![feature(iter_array_chunks)]

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,    
}

enum Operand {
    Value(u64),
    Old,
}

struct Monkey {
    number: usize,
    items: Vec<u64>,
    operator: Operator,
    operand: Operand,
    modulo: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

fn parse_nodes(input: &str) -> Vec<Monkey> {
    let result: Vec<Monkey> = input
        .lines()
        .array_chunks::<7>()
        .map(|item|{
            let number = item[0].split_whitespace().nth(1).expect("invalid input, monkey").
                split_once(':').expect("invalid input, monkey").0.parse::<u64>().expect("invalid input, monkey") as usize;
            let items = item[1].split_once(':').expect("invalid input, items").1.split(',').map(|text| {
                text.trim().parse::<u64>().expect("invalid input, items")
            }).collect::<Vec<u64>>();
            let operator: Operator = match item[2].split("  Operation: new = old ").nth(1).expect("invalid input").chars().next() {
                Some('+') => Operator::Add,
                Some('-') => Operator::Subtract,
                Some('*') => Operator::Multiply,
                Some('/') => Operator::Divide,
                Some(_) => todo!(), //invalid input
                None => todo!(), //invalid input
            };
            let operand: Operand = match item[2].split_whitespace().nth(5).expect("invalid input, operand").parse() {
                Ok(a) => Operand::Value(a),
                _ => Operand::Old,
            };
            let modulo = item[3].split_whitespace().nth(3).expect("invalid input, modulo").parse().expect("invalid input, modulo");
            let if_true = item[4].split_whitespace().nth(5).expect("invalid input, if true").parse().expect("invalid input, if true");
            let if_false = item[5].split_whitespace().nth(5).expect("invalid input, if false").parse().expect("invalid input, if false");
            Monkey{ number, items, operator, operand, modulo, if_true, if_false, inspections: 0 }
        }).collect::<Vec<Monkey>>();
    for (i, item) in result.iter().enumerate() {
        if item.number != i {
            panic!("input was invalid");
        }        
    }
    result
}

fn inspect_items(input: &mut Monkey) -> Vec<(usize, u64)> {
    let mut result = Vec::new();
    while let Some(item) = (*input).items.pop() {
        let mut new_item = match (*input).operator {
            Operator::Add => item + (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Subtract => item - (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Multiply => item * (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Divide => item / (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
        };
        new_item = new_item / 3;
        (*input).inspections += 1;
        match new_item % (*input).modulo == 0 {
            true => result.push(((*input).if_true, new_item)),
            false => result.push(((*input).if_false, new_item)),
        }
    };
    result
}

fn inspect_items2(input: &mut Monkey, rmdr_thm: u64) -> Vec<(usize, u64)> {
    let mut result = Vec::new();
    while let Some(item) = (*input).items.pop() {
        let mut new_item = match (*input).operator {
            Operator::Add => item + (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Subtract => item - (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Multiply => item * (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
            Operator::Divide => item / (match (*input).operand {
                Operand::Value(v) => v,
                Operand::Old => item,
            }),
        };
        (*input).inspections += 1;
        new_item = new_item % rmdr_thm;
        match new_item % (*input).modulo == 0 {
            true => result.push(((*input).if_true, new_item)),
            false => result.push(((*input).if_false, new_item)),
        }
    };
    result
}

fn insert_pairs(input: Vec<(usize, u64)>, list: &mut Vec<Monkey>) {
    for (destination, item) in input {
        list[destination].items.push(item);
    }
}

pub fn process_part1(input: &str) -> String {
    let mut list = parse_nodes(input);
    for _ in 0..20 { // rounds
        for i in 0..list.len() {
            let outgoing_pairs = inspect_items(&mut list[i]);
            insert_pairs(outgoing_pairs, &mut list);
        }
    }
    let mut inspections: Vec<u64> = list
        .iter()
        .map(|item| {
            item.inspections
        })
        .collect::<Vec<_>>();
    inspections.sort_by(|a, b| b.cmp(a));
    (inspections[0] * inspections[1]).to_string()
}

pub fn process_part2(input: &str) -> String {
    let first_few_primes: [u64; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mut list = parse_nodes(input);
    let remainder_theorem_magic_number: u64 = list
        .iter()
        .map(|item| {
            if !first_few_primes.contains(&item.modulo) {
                panic!("cannot do remainder theorem")
                // they could also be _relatively_ prime, but the inputs are all prime in this case
            }
            item.modulo
        }).product();
    for _ in 0..10000 { // rounds
        for i in 0..list.len() {
            let outgoing_pairs = inspect_items2(&mut list[i], remainder_theorem_magic_number);
            insert_pairs(outgoing_pairs, &mut list);
        }
    }
    let mut inspections: Vec<u64> = list
        .iter()
        .map(|item| {
            item.inspections
        })
        .collect::<Vec<_>>();
    inspections.sort_by(|a, b| b.cmp(a));
    (inspections[0] * inspections[1]).to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "10605");
        assert_eq!(process_part2(file.as_str()), "2713310158");
    }
}
