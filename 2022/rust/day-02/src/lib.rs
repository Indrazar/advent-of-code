pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|round| {
            if round.chars().count() != 3 {dbg!(format!("incorrect input format"));}
            round_calc(
                round.chars().nth(0).unwrap(),
                round.chars().nth(2).unwrap()
            )
        })
        .sum::<i32>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|round| {
            if round.chars().count() != 3 {dbg!(format!("incorrect input format"));}
            let elf: char = round.chars().nth(0).unwrap();
            let strategy: char = round.chars().nth(2).unwrap();
            let player: char = strategy_adj(elf, strategy);
            round_calc(elf, player)
        })
        .sum::<i32>();
    result.to_string()
}

fn strategy_adj(elf: char, strategy: char) -> char {
    //X lose
    //Y draw
    //Z win
    match strategy {
        'X' => match elf {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => todo!("shouldn't get here, strategy_adj, ? X")
        },
        'Y' => match elf {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => todo!("shouldn't get here, strategy_adj, ? Y")
        },
        'Z' => match elf {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => todo!("shouldn't get here, strategy_adj, ? Z")
        }
        _ => todo!("shouldn't get here, strategy_adj, _ ?")
    }
}

fn round_calc(elf: char, player: char) -> i32 {
    let shape_score: i32 = match player {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => todo!("shouldn't get here"),
    };
    let win_score: i32 = match (elf, player) {
        ('A','Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        (_, _) => 0,
    };
    let draw_score: i32 = if elf == shifted(player) {3} else {0};

    shape_score + win_score + draw_score
}

fn shifted(input: char) -> char {
    match input {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => '?'
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "15");
        assert_eq!(process_part2(file.as_str()), "12");
    }
}
