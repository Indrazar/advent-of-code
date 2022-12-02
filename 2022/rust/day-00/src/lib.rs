pub fn process_part1(input: &str) -> String {
    "works".to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "works");
    }
}
