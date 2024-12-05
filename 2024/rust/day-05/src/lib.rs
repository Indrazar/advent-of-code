use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    multi::{many1, many_till, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    first: i32,
    second: i32,
}

#[derive(Debug, Clone)]
struct PairRules {
    find_second: HashMap<i32, Vec<i32>>,
    find_first: HashMap<i32, Vec<i32>>,
}

type Updates = Vec<Vec<i32>>;

fn parse_page_pair(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (a, b)) = delimited(
        tag(""),
        separated_pair(complete::i32, tag("|"), complete::i32),
        tag(""),
    )(input)?;
    Ok((input, (a, b)))
}

fn parse_rules(input: &str) -> IResult<&str, PairRules> {
    let mut rules = PairRules {
        find_second: HashMap::new(),
        find_first: HashMap::new(),
    };
    let (input, pairs): (_, Vec<Pair>) =
        many1(many_till(anychar, parse_page_pair).map(|(_, (a, b))| Pair {
            first: a,
            second: b,
        }))(input)
        .expect("parse should succeed");
    for pair in pairs {
        match rules.find_first.get(&pair.second) {
            Some(vec) => {
                let mut new_vec = vec.clone();
                new_vec.push(pair.first);
                rules.find_first.insert(pair.second, new_vec);
            }
            None => {
                rules.find_first.insert(pair.second, vec![pair.first]);
            }
        }
        match rules.find_second.get(&pair.first) {
            Some(vec) => {
                let mut new_vec = vec.clone();
                new_vec.push(pair.second);
                rules.find_second.insert(pair.first, new_vec);
            }
            None => {
                rules.find_second.insert(pair.first, vec![pair.second]);
            }
        }
    }
    Ok((input, rules))
}

fn parse_updates(input: &str) -> IResult<&str, Updates> {
    separated_list1(line_ending, separated_list1(tag(","), complete::i32))(input)
}

fn parse(input: &str) -> IResult<&str, (PairRules, Updates)> {
    let (input, rules) = terminated(parse_rules, line_ending)(input).expect("parse should succeed");
    let (input, updates) =
        preceded(line_ending, parse_updates)(input).expect("parse should succeed");
    Ok((input, (rules, updates)))
}

fn is_before(left: i32, right: i32, rules: &PairRules) -> bool {
    match rules.find_second.get(&left) {
        Some(vec) => {
            if vec.contains(&right) {
                return true;
            }
        }
        None => {}
    }
    match rules.find_first.get(&right) {
        Some(vec) => {
            if vec.contains(&left) {
                return true;
            }
        }
        None => {}
    }
    return false;
}

fn check_rule(update: &Vec<i32>, rules: &PairRules) -> bool {
    for pair in update.windows(2) {
        if !is_before(pair[0], pair[1], rules) {
            return false;
        }
    }
    true
}

pub fn process_part1(input: &str) -> String {
    let mut middle_page_sum = 0;
    let (_, (rules, updates)) = parse(input).expect("parse should succeed");
    for update in updates {
        if check_rule(&update, &rules) {
            if update.len() % 2 != 1 {
                panic!("update lengths need to be odd")
            } else {
                middle_page_sum += update[update.len() / 2]
            }
        } else {
        }
    }
    middle_page_sum.to_string()
}

fn get_middle_of_correctly_sorted(update: &Vec<i32>, rules: &PairRules) -> i32 {
    let mut update = update.clone();
    let mut swap_counter = 0;
    let swap_limit = update.len() * update.len();
    while (!check_rule(&update, rules)) && (swap_counter < swap_limit) {
        for i in 1..update.len() {
            if !is_before(update[i - 1], update[i], rules) {
                update.swap(i - 1, i);
                break;
            } else {
            }
        }
        swap_counter += 1;
    }
    if swap_counter < swap_limit {
        return update[update.len() / 2];
    } else {
        dbg!(update);
        dbg!(swap_counter);
        panic!("hit swap limit")
    }
}

pub fn process_part2(input: &str) -> String {
    let mut middle_page_sum = 0;
    let (_, (rules, updates)) = parse(input).expect("parse should succeed");
    for update in updates {
        if !check_rule(&update, &rules) {
            if update.len() % 2 != 1 {
                panic!("update lists need to be odd lengths")
            } else {
                middle_page_sum += get_middle_of_correctly_sorted(&update, &rules)
            }
        } else {
        }
    }
    middle_page_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "143");
        assert_eq!(process_part2(file), "123");
    }
}
