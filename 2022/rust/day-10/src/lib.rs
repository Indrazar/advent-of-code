#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),  
}

fn gen_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .fold(Vec::new(), |mut acc, item| {
            let mut iter = item.split_whitespace();
            match iter.next() {
                Some("addx") => {
                    match iter.next() {
                        Some(number) => {
                            let parse_num = number.parse::<i64>().expect("requires a number after addx");
                            let x = Instruction::Addx(parse_num);
                            acc.push(x);
                            acc
                        },
                        None => todo!(), // invalid input
                    }
                },
                Some("noop") => {acc.push(Instruction::Noop); acc },
                Some(_) => todo!(), // invalid input
                None => todo!(), // invalid input
            }
        })
}

fn signal_strength(instructions: Vec<Instruction>, important_cycles: Vec<i64>) -> i64 {
    let mut current_cycle: i64 = 1;
    let mut x: i64 = 1;
    let mut signal_counter: i64 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                if important_cycles.contains(&current_cycle) {
                    signal_counter += current_cycle * x;
                }
                current_cycle += 1;
            }
            Instruction::Addx(val) => {
                if important_cycles.contains(&current_cycle) {
                    signal_counter += current_cycle * x;
                }
                current_cycle += 1;
                if important_cycles.contains(&current_cycle) {
                    signal_counter += current_cycle * x;
                }
                x += val;
                current_cycle += 1;
            },
        }
    }

    signal_counter
}

fn update_screen(cycle: i64, x: i64, screen: &mut Vec<Vec<char>>) {
    if (cycle % 40) == x || (cycle % 40)+1 == x || (cycle % 40)-1 == x {
        screen[((cycle/40) as usize)][((cycle%40) as usize)] = '#';                 
    }
}

fn draw_screen(instructions: Vec<Instruction>) -> String {
    let mut current_cycle: i64 = 0;
    let mut x: i64 = 1;
    let mut screen: Vec<Vec<char>> = Vec::new();
    screen.reserve(6);
    for _ in 0..6 {
        let mut row: Vec<char> = Vec::new();
        row.reserve(40);
        for _ in 0..40 {
            row.push('.');
        }
        screen.push(row);
    }

    update_screen(current_cycle, x, &mut screen);

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                update_screen(current_cycle, x, &mut screen);
                current_cycle += 1;
            }
            Instruction::Addx(val) => {
                update_screen(current_cycle, x, &mut screen);
                current_cycle += 1;
                update_screen(current_cycle, x, &mut screen);
                x += val;
                current_cycle += 1;
            },
        }
    }

    let mut result = String::new();
    for row in screen {
        for col in row {
            result.push(col)
        }
        result.push('\n')
    }
    // remove last \n
    result.pop();
    result

}

pub fn process_part1(input: &str) -> String {
    let ins = gen_instructions(input);
    let important_cycles = vec![20, 60, 100, 140, 180, 220];
    let result = signal_strength(ins, important_cycles);
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let ins = gen_instructions(input);
    let result = draw_screen(ins);
    result
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "13140");
        assert_eq!(process_part2(file.as_str()), 
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....")
    }
}
