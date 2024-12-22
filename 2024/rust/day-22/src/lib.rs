use std::collections::HashMap;

fn mix(current: usize, input: usize) -> usize {
    current ^ input
}

fn prune(current: usize) -> usize {
    current % 16777216
}

fn get_next(input: usize, map: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = map.get(&input) {
        return *v;
    } else {
        // mix: mult 64, then prune
        let step1 = prune(mix(input, input * 64));
        let step2 = prune(mix(step1, step1 / 32));
        let step3 = prune(mix(step2, step2 * 2048));
        map.insert(input, step3);
        step3
    }
}

pub fn process_part1(input: &str) -> String {
    let mut map = HashMap::new();
    let mut buyer_values: Vec<usize> = Vec::new();
    for line in input.lines() {
        buyer_values.push(line.parse().expect("there should be a number"));
    }
    let mut final_values: Vec<usize> = Vec::new();
    for buyer in buyer_values {
        let mut current = buyer;
        for _ in 0..2000 {
            current = get_next(current, &mut map);
        }
        final_values.push(current);
    }
    final_values.iter().sum::<usize>().to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut map = HashMap::new();
        let start = 123;
        let mut current = get_next(start, &mut map);
        let check = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for val in check {
            assert_eq!(current, val);
            current = get_next(current, &mut map);
        }
    }

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "37327623");
    }
}
