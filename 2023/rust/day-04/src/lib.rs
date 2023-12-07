fn count_sorted_matches(winning: Vec<u32>, picked: Vec<u32>) -> u32 {
    let mut matches = 0;
    for winner in winning.iter() {
        for pick in picked.iter() {
            if winner > pick {
                continue;
            } else if winner == pick {
                matches += 1;
            } else {
                break;
            }
        }
    }
    matches
}

fn process_game_winner_count(game: &str) -> u32 {
    let mut winning_numbers = game
        .split(':')
        .nth(1)
        .expect("should be input after :")
        .split('|')
        .nth(0)
        .expect("should be input before |")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    winning_numbers.sort();
    let mut picked_numbers = game
        .split(':')
        .nth(1)
        .expect("should be input after :")
        .split('|')
        .nth(1)
        .expect("should be input after |")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    picked_numbers.sort();
    count_sorted_matches(winning_numbers, picked_numbers)
}

fn calculate_score(wins: u32) -> u32 {
    if wins == 0 {
        return 0;
    } else {
        u32::pow(2, wins - 1)
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|game| process_game_winner_count(game))
        .map(|wins| calculate_score(wins))
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut copy_total: Vec<u32> = Vec::default();
    for _ in input.lines() {
        copy_total.push(1);
    }
    let mut game_id: usize = 1;
    for game in input.lines() {
        let wins: usize = process_game_winner_count(game)
            .try_into()
            .expect("u32 should fit in usize");
        if wins > 0 {
            for new_copy in (game_id + 1)..(game_id + 1 + wins) {
                if (new_copy - 1) < copy_total.len() {
                    // vec is 0 indexed so we need to offset the id
                    copy_total[new_copy - 1] += copy_total[game_id - 1]; // vec is 0 indexed so we need to offset the id
                }
            }
        }
        game_id += 1; // move to next game
    }
    copy_total.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "13");
        assert_eq!(process_part2(file.as_str()), "30");
    }
}
