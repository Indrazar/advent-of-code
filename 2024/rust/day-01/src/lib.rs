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

fn split_numbers(input: &str) -> (i32, i32) {
    let mut input_iter = input.split("  ");
    let res1: i32 = input_iter
        .next()
        .expect(format!("invalid input, nothing: {}", input).as_str())
        .trim()
        .parse::<i32>()
        .expect(format!("invalid input, not an int: {}", input).as_str());
    let res2: i32 = input_iter
        .next()
        .expect(format!("invalid input, nothing: {}, res1: {}", input, res1).as_str())
        .trim()
        .parse::<i32>()
        .expect(format!("invalid input, not an int: {}, res1: {}", input, res1).as_str());
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

fn get_count(list: &Vec<i32>, val: i32) -> i32 {
    if &val < list.first().expect("list should not be empty") {
        0
    } else if &val > list.last().expect("list should not be empty") {
        0
    } else {
        list.iter()
            .filter(|x| *x == &val)
            .count()
            .try_into()
            .expect("count is larger than i32")
    }
}

pub fn process_part2(input: &str) -> String {
    let (list1, mut list2) = input
        .lines()
        .map(split_numbers)
        .collect::<(Vec<i32>, Vec<i32>)>();
    list2.sort();
    let mut total = 0;
    for l1 in list1 {
        total += l1 * get_count(&list2, l1);
    }
    format!("{total}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_split() {
        let testval: &str = "3   4";
        assert_eq!(split_numbers(testval), (3, 4))
    }

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "11");
    }
    #[test]
    fn test_input2() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part2(file.as_str()), "31");
    }
}
