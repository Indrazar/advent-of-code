use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> String {
    let char_holder = input
        .chars()
        .collect::<Vec<char>>();
    let sequence = char_holder
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();
    (sequence.0 + 4).to_string()
}

pub fn process_part2(input: &str) -> String {
    let char_holder = input
        .chars()
        .collect::<Vec<char>>();
    let sequence = char_holder
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();
    (sequence.0 + 14).to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file1 = fs::read_to_string("./test-input-1.txt").unwrap();
        let file2 = fs::read_to_string("./test-input-2.txt").unwrap();
        let file3 = fs::read_to_string("./test-input-3.txt").unwrap();
        let file4 = fs::read_to_string("./test-input-4.txt").unwrap();
        assert_eq!(process_part1(file1.as_str()), "5");
        assert_eq!(process_part1(file2.as_str()), "6");
        assert_eq!(process_part1(file3.as_str()), "10");
        assert_eq!(process_part1(file4.as_str()), "11");
        assert_eq!(process_part2(file1.as_str()), "23");
        assert_eq!(process_part2(file2.as_str()), "23");
        assert_eq!(process_part2(file3.as_str()), "29");
        assert_eq!(process_part2(file4.as_str()), "26");
    }
}

