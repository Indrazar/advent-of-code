pub fn process_part1(input: &str) -> String {
    let mut invalid_ids: Vec<u64> = Vec::new();
    for group in input.split(',') {
        let mut range = group.split("-");
        let left: u64 = range.next().unwrap().parse().unwrap();
        let right: u64 = range.next().unwrap().parse().unwrap();
        for value in left..=right {
            let stringed = format!("{value}");
            if (stringed.len() % 2) == 0 {
                let (str_left, str_right) = stringed.split_at(stringed.len() / 2);
                if str_left == str_right {
                    invalid_ids.push(value);
                }
            }
        }
    }
    invalid_ids.iter().sum::<u64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut invalid_ids: Vec<u64> = Vec::new();
    for group in input.split(',') {
        let mut range = group.split("-");
        let left: u64 = range.next().unwrap().parse().unwrap();
        let right: u64 = range.next().unwrap().parse().unwrap();
        for value in left..=right {
            let stringed = format!("{value}");
            'windows: for window_size in 1..=stringed.len() / 2 {
                if stringed.len() % window_size == 0 {
                    let str_chunks: Vec<String> = stringed
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(window_size)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect();
                    for i in 0..str_chunks.len() - 1 {
                        if str_chunks[i] != str_chunks[i + 1] {
                            continue 'windows;
                        }
                    }
                    invalid_ids.push(value);
                    break;
                }
            }
        }
    }
    invalid_ids.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "1227775554");
        assert_eq!(process_part2(file), "4174379265");
    }
}
