pub fn process_part1(input: &str) -> String {
    let mut first_ch = ' ';
    let mut final_ch = ' ';
    let all_chars: Vec<char> = input.chars().collect();
    let mut sum = 0;
    for (i, ch) in input.chars().enumerate() {
        if i == 0 {
            first_ch = ch;
        } else {
            final_ch = ch;
            if ch == all_chars[i - 1] {
                sum += ch.to_digit(10).expect("should parse");
            }
        }
    }
    if final_ch == first_ch {
        sum += final_ch.to_digit(10).expect("should parse");
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let all_chars: Vec<char> = input.chars().collect();
    let list_len = all_chars.len();
    let halfway_around = list_len / 2;
    let mut sum = 0;
    for (i, ch) in input.chars().enumerate() {
        let lookup_index = match (i + halfway_around) >= list_len {
            true => i + halfway_around - list_len,
            false => i + halfway_around,
        };
        if ch == all_chars[lookup_index] {
            sum += ch.to_digit(10).expect("should parse");
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1("1122"), "3");
        assert_eq!(process_part1("1111"), "4");
        assert_eq!(process_part1("1234"), "0");
        assert_eq!(process_part1("91212129"), "9");
        assert_eq!(process_part2("1212"), "6");
        assert_eq!(process_part2("1221"), "0");
        assert_eq!(process_part2("123425"), "4");
        assert_eq!(process_part2("123123"), "12");
        assert_eq!(process_part2("12131415"), "4");
    }
}
