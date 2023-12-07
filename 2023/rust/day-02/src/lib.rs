const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

fn process_game_type1(input: &str) -> u32 {
    let mut rounds = input.split(&[':', ';'][..]);
    let id = rounds
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    for round in rounds {
        let round = round.trim();
        let pull_segment = round.split(", ");
        for segment in pull_segment {
            let (count, color) = (
                segment.split_whitespace().nth(0).unwrap(),
                segment.split_whitespace().nth(1).unwrap(),
            );
            match color {
                "red" => {
                    if count.parse::<u32>().unwrap() > RED_LIMIT {
                        return 0;
                    }
                }
                "green" => {
                    if count.parse::<u32>().unwrap() > GREEN_LIMIT {
                        return 0;
                    }
                }
                "blue" => {
                    if count.parse::<u32>().unwrap() > BLUE_LIMIT {
                        return 0;
                    }
                }
                _ => panic!("input is malformed"),
            }
        }
    }
    id
}

fn process_game_type2(input: &str) -> u32 {
    let mut needed_red = 0;
    let mut needed_green = 0;
    let mut needed_blue = 0;
    let mut rounds = input.split(&[':', ';'][..]);
    let _id = rounds
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    for round in rounds {
        let round = round.trim();
        let pull_segment = round.split(", ");
        for segment in pull_segment {
            let (count, color) = (
                segment.split_whitespace().nth(0).unwrap(),
                segment.split_whitespace().nth(1).unwrap(),
            );
            match color {
                "red" => {
                    let reds = count.parse::<u32>().unwrap();
                    if reds > needed_red {
                        needed_red = reds;
                    }
                }
                "green" => {
                    let greens = count.parse::<u32>().unwrap();
                    if greens > needed_green {
                        needed_green = greens;
                    }
                }
                "blue" => {
                    let blues = count.parse::<u32>().unwrap();
                    if blues > needed_blue {
                        needed_blue = blues;
                    }
                }
                _ => panic!("input is malformed"),
            }
        }
    }
    needed_red * needed_green * needed_blue
}

pub fn process_part1(input: &str) -> String {
    let mut possible_id_sum = 0;
    for game in input.lines() {
        possible_id_sum += process_game_type1(game);
    }
    format!("{possible_id_sum}")
}

pub fn process_part2(input: &str) -> String {
    let mut sum = 0;
    for game in input.lines() {
        sum += process_game_type2(game);
    }
    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "8");
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part2(file.as_str()), "2286")
    }
}
