use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{self, anychar},
    error::Error,
    multi::{many1, many_till, separated_list1},
    sequence::{delimited, separated_pair},
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
    let (_, pairs): (_, Vec<Pair>) =
        many1(many_till(anychar, parse_page_pair).map(|(_, (a, b))| Pair {
            first: a,
            second: b,
        }))(input)?;
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

fn parse_single_update(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, vec) = separated_list1(tag(","), complete::i32)(input)?;
    Ok((input, vec))
}

fn parse_updates(input: &str) -> IResult<&str, Updates> {
    separated_list1(tag("\n"), parse_single_update)(input)
}

fn parse(input: &str) -> IResult<&str, (PairRules, Updates)> {
    let (update_input, rules_input) =
        take_until::<_, _, Error<_>>("\n\n")(input).expect("there should be 2 line breaks");
    let (_, rules) = parse_rules(rules_input)?;
    let (update_input, _) = take(2usize)(update_input)?; // remove the first 2 \n\n characters
    let (_, updates) = parse_updates(update_input)?;
    Ok(("", (rules, updates)))
}

fn is_before(left: i32, right: i32, rules: &PairRules, seen_values: &Vec<i32>) -> bool {
    match rules.find_second.get(&left) {
        Some(vec) => {
            for page in vec {
                if *page == right {
                    //println!("left: {left}, right: {right}: find_second, less");
                    return true;
                }
            }
        }
        None => {}
    }
    match rules.find_first.get(&right) {
        Some(vec) => {
            for page in vec {
                if *page == left {
                    //println!("left: {left}, right: {right}: find_first, less");
                    return true;
                }
            }
        }
        None => {}
    }
    //println!("left: {left}, right: {right}: idk lol");
    let mut seen_values_backwards = seen_values.clone();
    seen_values_backwards.reverse();
    let mut seen_values_reduced = seen_values.clone();
    let last_lookup = match seen_values_reduced.pop() {
        Some(val) => val,
        None => {
            //println!("left: {left}, right: {right}: out of values to pop: false");
            return false;
        }
    };
    for value in seen_values_backwards {
        if !is_before(last_lookup, value, rules, &seen_values_reduced) {
            //println!("iterating backwards recursively found that last_lookup: {last_lookup} was after value: {value}");
            return false;
        }
    }
    panic!("should never get here");
}

fn check_rule(update: &Vec<i32>, rules: &PairRules) -> bool {
    let mut seen_values: Vec<i32> = Vec::new();
    for pair in update.windows(2) {
        if !is_before(pair[0], pair[1], rules, &seen_values) {
            return false;
        }
        seen_values.push(pair[0]);
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
                //println!(
                //    "we have found a valid update, middle value is: {}",
                //    update[update.len() / 2]
                //);
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
    let mut seen_values: Vec<i32> = Vec::new();
    while (!check_rule(&update, rules)) && (swap_counter < update.len() * update.len()) {
        for i in 1..update.len() {
            if !is_before(update[i - 1], update[i], rules, &seen_values) {
                let left = update[i - 1];
                let right = update[i];
                //println!("swapping {left} and {right}");
                update[i - 1] = right;
                update[i] = left;
                break;
            } else {
                seen_values.push(update[i - 1]);
            }
        }
        swap_counter += 1;
    }
    if swap_counter < update.len() * update.len() {
        return update[update.len() / 2];
    } else {
        dbg!(update);
        dbg!(swap_counter);
        dbg!(seen_values);
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
