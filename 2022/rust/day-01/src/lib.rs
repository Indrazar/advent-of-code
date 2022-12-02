pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|item| item.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut bundles: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|item| item.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect();
    bundles.sort_by(|a,b| b.cmp(a));
    let result: u32 = bundles.iter().take(3).sum();
    result.to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "24000");
        assert_eq!(process_part2(file.as_str()), "45000");
    }
}
