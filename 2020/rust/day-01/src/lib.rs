pub fn process_part1(input: &str) -> String {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().expect("parse should succeed"));
    }
    for i in 1..numbers.len() {
        for j in 0..i {
            if numbers[i] + numbers[j] == 2020 {
                return (numbers[i] * numbers[j]).to_string();
            }
        }
    }
    "no solution found".to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().expect("parse should succeed"));
    }
    for i in 2..numbers.len() {
        for j in 1..i {
            for k in 0..j {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return (numbers[i] * numbers[j] * numbers[k]).to_string();
                }
            }
        }
    }
    "no solution found".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "514579");
        assert_eq!(process_part2(file), "241861950");
    }
}
