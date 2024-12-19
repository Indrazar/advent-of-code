pub fn process_part1(input: &str) -> String {
    let mut total = 0;
    for ch in input.chars() {
        match ch {
            '(' => {
                total += 1;
            }
            ')' => {
                total -= 1;
            }
            _ => {}
        }
    }
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut total = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => {
                total += 1;
            }
            ')' => {
                total -= 1;
            }
            _ => {}
        }
        if total == -1 {
            return (i + 1).to_string();
        }
    }
    total.to_string()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_input() {
//         let file = include_str!("../test-input-1.txt");
//         assert_eq!(process_part1(file), "7");
//         //assert_eq!(process_part2(file), "5");
//     }
// }
