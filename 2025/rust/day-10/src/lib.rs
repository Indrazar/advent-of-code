use itertools::Itertools;

struct Row {
    lights: usize,
    goal: usize,
    buttons: Vec<usize>,
}

fn fast_parse_line(input: &str) -> Row {
    let mut lights: usize = 0;
    let mut goal: usize = 0;
    let mut buttons = Vec::new();
    for segment in input.split(' ') {
        let mut segment_itr = segment.chars();
        match segment_itr.next() {
            Some('[') => {
                for (bit, char) in segment_itr.enumerate() {
                    // println!(
                    //     "bit: {bit}, char: {char}, goal_inc: {}",
                    //     2u32.pow(bit.try_into().unwrap()) as usize
                    // );
                    match char {
                        '.' => {}
                        '#' => goal += 2u32.pow(bit.try_into().unwrap()) as usize,
                        ']' => {
                            lights = bit;
                            break;
                        }
                        _ => panic!("invalid character parsing buttons: {char}, {segment}"),
                    }
                }
            }
            Some('(') => {
                let mut button: usize = 0;
                for number in segment.split(&['(', ',', ')'][..]) {
                    if number.len() == 0 {
                        continue;
                    }
                    let pos: u32 = number.parse().expect("should be a number");
                    button += 2u32.pow(pos) as usize;
                }
                buttons.push(button);
            }
            Some('{') => {}
            _ => {
                panic!("invalid character at start of segment: {segment}")
            }
        }
    }
    if lights == 0 && buttons.len() == 0 {
        panic!(
            "could not parse row, lights: {lights}, buttons: {}",
            buttons.len()
        )
    }
    // print!("row complete: lights: {lights}, goal: {goal}, buttons: ");
    // for button in &buttons {
    //     print!("{button}, ");
    // }
    // println!();
    Row {
        lights,
        goal,
        buttons,
    }
}

fn solve_row(input: &Row) -> usize {
    for presses in 1..=input.buttons.len() * 5 {
        for combination in input.buttons.iter().combinations_with_replacement(presses) {
            let mut current: usize = 0;
            for num in combination {
                current ^= num;
                if current == input.goal {
                    return presses;
                }
            }
        }
    }
    print!(
        "row failed: lights: {}, goal: {}, buttons: ",
        input.lights, input.goal
    );
    for button in &input.buttons {
        print!("{button}, ");
    }
    println!();
    panic!("could not solve row")
}

pub fn process_part1(input: &str) -> String {
    let machines: Vec<Row> = input.lines().map(|x| fast_parse_line(x)).collect();
    machines
        .iter()
        .map(|x| solve_row(x))
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "7");
    }
}
