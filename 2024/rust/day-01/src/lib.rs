use std::collections::HashMap;
use std::iter::zip;

pub trait SneakPrintStr {
    fn sneak_print(&self) -> &str;
}
impl SneakPrintStr for str {
    fn sneak_print(&self) -> &str {
        println!("{:?}", self);
        self
    }
}

fn split_numbers(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let res1: i32 = parts[0].parse().unwrap();
    let res2: i32 = parts[1].parse().unwrap();
    (res1, res2)
}

fn split_numbers_with_count(line: &str, counts: &mut HashMap<i32, i32>) -> (i32, i32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let res1: i32 = parts[0].parse().unwrap();
    let res2: i32 = parts[1].parse().unwrap();
    counts
        .entry(res2)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    (res1, res2)
}

pub fn process_part1(input: &str) -> String {
    let (mut list1, mut list2) = input
        .lines()
        .map(split_numbers)
        .collect::<(Vec<i32>, Vec<i32>)>();
    list1.sort();
    list2.sort();
    if list1.len() != list2.len() {
        panic!("list sizes do not match, check input")
    }
    let mut total = 0;
    for (l1, l2) in zip(list1, list2) {
        total += (l1 - l2).abs()
    }
    format!("{total}")
}

pub fn process_part2(input: &str) -> String {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let (list1, mut list2) = input
        .lines()
        .map(|line| split_numbers_with_count(line, &mut counts))
        .collect::<(Vec<i32>, Vec<i32>)>();
    list2.sort();
    let mut total = 0;
    for l1 in list1 {
        total += l1 * counts.get(&l1).unwrap_or(&0);
    }
    format!("{total}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let testval: &str = "3   4";
        assert_eq!(split_numbers(testval), (3, 4))
    }

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "11");
        assert_eq!(process_part2(file), "31");
    }
}
