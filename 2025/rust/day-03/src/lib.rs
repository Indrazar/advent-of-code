fn find_largest_pos_with_padding(
    input: &Vec<u64>,
    full: bool,
    start: usize,
    remaining: usize,
) -> usize {
    let mut pos_max: usize = 0;
    let mut max: u64 = 0;
    if full {
        for pos in start..(input.len() - (remaining)) {
            if input[pos] > max {
                max = input[pos];
                pos_max = pos;
                if max == 9 {
                    break;
                }
            }
        }
    } else {
        for pos in (start + 1)..(input.len() - (remaining)) {
            if input[pos] > max {
                max = input[pos];
                pos_max = pos;
                if max == 9 {
                    break;
                }
            }
        }
    }
    pos_max
}

fn process_max_value(input: &str, batteries: usize) -> u64 {
    let bank: Vec<u64> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    let mut digits: Vec<u64> = Vec::new();
    let mut total = 0;
    let mut index = 0;
    index = find_largest_pos_with_padding(&bank, true, index, batteries - 1);
    digits.push(bank[index]);
    total += bank[index] * (10u64.pow((batteries - 1).try_into().unwrap()));
    for remain in (0..batteries - 1).rev() {
        index = find_largest_pos_with_padding(&bank, false, index, remain);
        digits.push(bank[index]);
        total += bank[index] * (10u64.pow(remain.try_into().unwrap()));
    }
    //println!("digits length: {}", digits.len());
    //for d in digits {
    //    print!("{d}");
    //}
    //print!("\n");
    //println!("total: {total}");
    total
}

pub fn process_part1(input: &str) -> String {
    let mut batteries: Vec<u64> = Vec::new();
    for line in input.lines() {
        batteries.push(process_max_value(line, 2))
    }
    batteries.iter().sum::<u64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut batteries: Vec<u64> = Vec::new();
    for line in input.lines() {
        batteries.push(process_max_value(line, 12))
    }
    batteries.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "357");
        assert_eq!(process_part2(file), "3121910778619");
    }
}
