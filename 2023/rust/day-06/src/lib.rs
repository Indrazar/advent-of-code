fn determine_win_strategy_count(time: u64, distance: u64) -> u64 {
    let mut wins = 0;
    for i in 0..time {
        if i * (time - i) > distance {
            wins += 1;
        }
    }
    wins
}

pub fn process_part1(input: &str) -> String {
    let times: Vec<u64> = input
        .lines()
        .nth(0)
        .expect("some input")
        .split("Time:")
        .nth(1)
        .expect("need times")
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().expect("times should be numbers"))
        .collect::<Vec<u64>>();
    let distances: Vec<u64> = input
        .lines()
        .nth(1)
        .expect("more input")
        .split("Distance:")
        .nth(1)
        .expect("need distances")
        .split_ascii_whitespace()
        .map(|dist| dist.parse::<u64>().expect("distances should be numbers"))
        .collect::<Vec<u64>>();
    dbg!(&times);
    dbg!(&distances);
    std::iter::zip(times, distances)
        .map(|(time, distance)| determine_win_strategy_count(time, distance))
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let time: u64 = input
        .lines()
        .nth(0)
        .expect("some input")
        .split("Time:")
        .nth(1)
        .expect("need times")
        .replace(" ", "")
        .parse::<u64>()
        .expect("time should a number");
    let distance = input
        .lines()
        .nth(1)
        .expect("more input")
        .split("Distance:")
        .nth(1)
        .expect("need distances")
        .replace(" ", "")
        .parse::<u64>()
        .expect("distance should a number");
    dbg!(&time);
    dbg!(&distance);
    determine_win_strategy_count(time, distance).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "288");
        assert_eq!(process_part2(file.as_str()), "71503");
    }
}
