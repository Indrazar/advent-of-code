use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    sync::{Arc, Mutex},
};

use itertools::Itertools;
use rayon::prelude::*;

struct P1Row {
    lights: usize,
    goal: usize,
    buttons: Vec<usize>,
}

fn fast_parse_line(input: &str) -> P1Row {
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
                    if number.is_empty() {
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
    if lights == 0 && buttons.is_empty() {
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
    P1Row {
        lights,
        goal,
        buttons,
    }
}

fn p1_solve_row(input: &P1Row) -> usize {
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
    panic!("could not solve row");
}

pub fn process_part1(input: &str) -> String {
    let machines: Vec<P1Row> = input.lines().map(fast_parse_line).collect();
    machines.iter().map(p1_solve_row).sum::<usize>().to_string()
}

struct P2Row {
    goal: Vec<usize>,
    buttons: Vec<Vec<usize>>,
}

fn p2_parse_line(input: &str) -> P2Row {
    let mut goal: Vec<usize> = Vec::new();
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    for segment in input.split(' ') {
        let mut segment_itr = segment.chars();
        match segment_itr.next() {
            Some('[') => {}
            Some('(') => {
                let mut button: Vec<usize> = Vec::new();
                for number in segment.split(&['(', ',', ')'][..]) {
                    if number.is_empty() {
                        continue;
                    }
                    button.push(number.parse().expect("should be a number"));
                }
                buttons.push(button);
            }
            Some('{') => {
                for number in segment.split(&['{', ',', '}'][..]) {
                    if number.is_empty() {
                        continue;
                    }
                    goal.push(number.parse().expect("should be a number"));
                }
            }
            _ => {
                panic!("invalid character at start of segment: {segment}")
            }
        }
    }
    if goal.is_empty() && buttons.is_empty() {
        panic!(
            "could not parse row, goal: {}, buttons: {}",
            goal.len(),
            buttons.len()
        )
    }
    // print!("row complete: lights: {lights}, goal: {goal}, buttons: ");
    // for button in &buttons {
    //     print!("{button}, ");
    // }
    // println!();
    P2Row { goal, buttons }
}

fn push_button(counter: &mut [usize], button: &[usize], press_count: usize) -> bool {
    for target in button {
        if counter[*target] < press_count {
            return false;
        } else {
            counter[*target] -= press_count;
        }
    }
    true
}

fn get_first_constraint(
    current: &[usize],
    all_buttons: &[Vec<usize>],
) -> Option<(usize, Vec<usize>)> {
    let mut constraints: Vec<(usize, Vec<usize>)> = Vec::with_capacity(current.len());
    for (counter, current_counter) in current.iter().enumerate() {
        let mut buttons: Vec<usize> = Vec::new();
        for (button_number, button) in all_buttons.iter().enumerate() {
            if button.contains(&counter) && *current_counter > 0 {
                buttons.push(button_number)
            }
        }
        if buttons.is_empty() {
            continue;
        } else {
            constraints.push((counter, buttons))
        }
    }
    constraints.sort_by_key(|x| x.1.len());
    // for counter in &current {
    //     print!("{counter},")
    // }
    // println!();
    //println!("constraints: {constraints:?}");
    constraints.first().cloned()
}

fn reduce(
    current: Vec<usize>,
    presses: usize,
    all_buttons: &Vec<Vec<usize>>, //sorted_buttons: &Vec<Vec<usize>>,
    upper_level_mins: Option<Vec<Arc<Mutex<usize>>>>,
) -> usize {
    if current.iter().sum::<usize>() == 0 {
        return presses;
    }
    // for counter in &current {
    //     print!("{counter},")
    // }
    // println!();
    if let Some((counter, constraint)) = get_first_constraint(&current, all_buttons) {
        //println!("counter: {counter}, first constraint: {constraint:?}, combos: ");
        // start with things only affected by small subsets // button_number, button
        let combos = constraint
            .iter()
            .combinations_with_replacement(current[counter]);
        //.collect::<Vec<Vec<&usize>>>();
        // for combo in combos.clone() {
        //     println!("{combo:?}");
        // }
        // println!();
        //let presses_so_far = combos
        let guarded_current_level_min = Arc::new(Mutex::new(usize::MAX));
        let upper_level_mins = match upper_level_mins {
            Some(mut x) => {
                x.push(guarded_current_level_min.clone());
                x
            }
            None => vec![guarded_current_level_min.clone()],
        };
        combos
            .par_bridge()
            .map(|combination| {
                let mut local_current = current.clone();
                let mut local_presses = presses;
                for button_number in combination {
                    //println!("pressing button: {button_number}");
                    local_presses += 1;
                    if !push_button(&mut local_current, &all_buttons[*button_number], 1) {
                        return usize::MAX;
                    }
                }
                // for (level, min) in upper_level_mins.iter().enumerate() {
                for min in upper_level_mins.iter() {
                    let min_value = min
                        .lock()
                        .expect("threads shouldn't panic while holding this");
                    if local_presses >= *min_value {
                        //println!("did not recurse due to smaller min at level: {level}:{min_value} ; current level is: {}:{local_presses}", upper_level_mins.len());
                        return usize::MAX;
                    }
                }
                reduce(
                    local_current,
                    local_presses,
                    all_buttons,
                    Some(upper_level_mins.clone()),
                )
            })
            .inspect(|res| {
                let mut min = guarded_current_level_min
                    .lock()
                    .expect("threads shouldn't panic while holding this");
                if *res < *min {
                    *min = *res;
                }
            })
            .min()
            .unwrap_or(usize::MAX)
        //println!("presses_so_far: {presses_so_far}");
        //presses_so_far
    } else {
        panic!("should always have a button to hit");
    }
}

fn p2_solve_row(/*, row_num: usize,*/ input: &P2Row) -> usize {
    let current = input.goal.clone();
    reduce(current, 0, &input.buttons, None)
}

pub fn process_part2(input: &str) -> String {
    let machines: Vec<P2Row> = input.lines().map(p2_parse_line).collect();
    let file_name = format!("{}-checkpoint.txt", machines.len());
    let mut checkpoint_file = match File::options().read(true).write(true).open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            println!("error opening checkpoint file: {e}");
            match File::create(&file_name) {
                Ok(file) => file,
                Err(e) => panic!("{e}: could not even create checkpoint file"),
            }
        }
    };
    let mut checkpoint_data: String = String::new();
    match checkpoint_file.read_to_string(&mut checkpoint_data) {
        Ok(_) => {}
        Err(e) => panic!("{e}: could not read file to string"),
    };
    let mut solutions: BTreeMap<usize, usize> = BTreeMap::new();
    for data in checkpoint_data.lines() {
        let (row_id, solution) = data
            .split_once(':')
            .expect("if there are rows it should have data");
        solutions.insert(
            row_id.parse().expect("row id should be a number"),
            solution.parse().expect("solution should be a number"),
        );
    }
    let guarded_file: Arc<Mutex<File>> = Arc::new(Mutex::new(checkpoint_file));
    //let guarded_soltuions: Arc<Mutex<BTreeMap<usize, usize>>> = Arc::new(Mutex::new(solutions));
    machines
        .into_par_iter()
        .enumerate()
        .map(|(row_number, row)| match solutions.get(&row_number) {
            Some(count) => *count,
            None => {
                let count = p2_solve_row(&row);
                println!("row {row_number} solved: {count}");
                let mut shared_file = guarded_file.lock().unwrap();
                let output_str = format!("{row_number}:{count}\n");
                match shared_file.write_all(output_str.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{e}: could not save line to file: {output_str}")
                    }
                };
                count
            }
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "7");
        assert_eq!(process_part2(file), "33");
    }
}
