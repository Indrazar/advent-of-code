pub fn process_part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            if line.chars().count() % 2 == 1 {
                panic!("odd number of objects");
            }
            let (first, second) = line.split_at(line.chars().count() / 2);
            let collision: char = {
                let mut found: Option<char> = Option::None;
                for letter in first.chars() {
                    for x in second.chars() {
                        if letter == x {
                            found = Some(letter);
                            break;
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                }
                found.unwrap()
            };
            map_priority(collision)
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut counter = 0;
    let result: usize = input
        .split(|letter| {
            if letter == '\n' {
                counter += 1;
                if counter == 3 {
                    counter = 0;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .map(|group| {
            if group.split_once('\n').is_none() {
                return 0;
            }
            let (first, rest): (&str, &str) = group.split_once('\n').unwrap();
            if rest.split_once('\n').is_none() {
                return 0;
            }
            let (second, third) = rest.split_once('\n').unwrap();
            let collision: char = {
                let mut found: Option<char> = Option::None;
                for letter in first.chars() {
                    for x in second.chars() {
                        if letter == x {
                            for y in third.chars() {
                                if letter == y {
                                    found = Some(letter);
                                    break;
                                }
                            }
                            if found.is_some() {
                                break;
                            }
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                }
                found.unwrap()
            };
            map_priority(collision)
        })
        .sum();
    result.to_string()
}

fn map_priority(input: char) -> usize {
    if input as usize > 96 {
        input as usize - 'a' as usize + 1
    } else {
        input as usize - 'A' as usize + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "157");
        assert_eq!(process_part2(file.as_str()), "70");
    }
}
