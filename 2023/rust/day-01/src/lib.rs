pub fn process_part1(input: &str) -> String {
    let value = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|symbol| symbol.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .map(|line| match (line.first(), line.last()) {
            (Some(a), Some(b)) => {
                //println!("{}", a * 10 + b);
                a * 10 + b
            }
            _ => 0,
        })
        .sum::<u32>();
    format!("{value}")
}

pub fn process_part2(input: &str) -> String {
    let value = input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };
                result.to_digit(10)
            });
            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum::<u32>();
    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "142");
        let file2 = fs::read_to_string("./test-input-2.txt").unwrap();
        assert_eq!(process_part2(file2.as_str()), "281");
    }
}
