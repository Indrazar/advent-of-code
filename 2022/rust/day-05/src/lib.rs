use nom::{bytes::complete::tag, character::complete, *};

struct MoveLine {
    count: u32,
    start: u32,
    end: u32
}

fn parse_crate_lines(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let tower_count = (input.find('\n').unwrap() + 1) / 4;
    for _ in 0..tower_count {
        result.push(Vec::new());
    }
    for line in input.lines() {
        for i in 0..tower_count {
            let letter = line.chars().nth(1+4*i).unwrap();
            if letter != ' ' {
                result[i].push(letter);
            }
        }
    }
    result
}

fn parse_single_move(input: &str) -> IResult<&str, MoveLine> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, start) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, end) = complete::u32(input)?;
    Ok((input, MoveLine{count, start: start-1, end: end-1}))
}

fn parse_move_lines(input: &str) -> Vec<MoveLine> {
    let mut result: Vec<MoveLine> = Vec::new();
    for line in input.lines() {
        if let Ok((_, move_line)) = parse_single_move(line) {
            result.push(move_line)
        }
    }
    result
}

pub fn process_part1(input: &str) -> String {
    let (crate_data, remainder) = input.split_at(input.find(" 1   2   3").unwrap());
    let crate_data = crate_data.trim_end();
    let (_, move_data) = remainder.split_at(remainder.find("\n\n").unwrap());
    let move_data = move_data.trim();
    let mut cargo = parse_crate_lines(crate_data);
    let moves = parse_move_lines(move_data);
    for crane_move in moves {
        for _ in 0..crane_move.count {
            let item = cargo[crane_move.start as usize].remove(0);
            cargo[crane_move.end as usize].insert(0, item);
        }
    }
    let mut result = String::new();
    for cargo_box in cargo {
        if !cargo_box.is_empty() {
            result.push(cargo_box[0])
        }
    }
    result
}

pub fn process_part2(input: &str) -> String {
    let (crate_data, remainder) = input.split_at(input.find(" 1   2   3").unwrap());
    let crate_data = crate_data.trim_end();
    let (_, move_data) = remainder.split_at(remainder.find("\n\n").unwrap());
    let move_data = move_data.trim();
    let mut cargo = parse_crate_lines(crate_data);
    let moves = parse_move_lines(move_data);
    for crane_move in moves {
        let mut items: Vec<char> = Vec::new();
        for _ in 0..crane_move.count {
            items.push(cargo[crane_move.start as usize].remove(0));
        }
        items.reverse();
        for item in items {
            cargo[crane_move.end as usize].insert(0, item);
        }
    }
    let mut result = String::new();
    for cargo_box in cargo {
        if !cargo_box.is_empty() {
            result.push(cargo_box[0])
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "CMZ");
        assert_eq!(process_part2(file.as_str()), "MCD");
    }
}
