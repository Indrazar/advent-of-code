use itertools::Itertools;

enum Mode {
    Added,
    Multiplied,
}

pub fn process_part1(input: &str) -> String {
    let mut operand_array: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Mode> = Vec::new();
    for line in input.lines() {
        let mut operandn: Vec<i64> = Vec::new();
        for text in line.split_ascii_whitespace() {
            match text.parse::<i64>() {
                Ok(val) => operandn.push(val),
                Err(_) => match text {
                    "*" => operators.push(Mode::Multiplied),
                    "+" => operators.push(Mode::Added),
                    _ => panic!("invalid operator: {text}"),
                },
            }
        }
        // for op in &operandn {
        //     print!("{op} ")
        // }
        //print!("\n");
        if operandn.len() > 0 {
            operand_array.push(operandn);
        }
    }
    // for op in &operators {
    //     match op {
    //         Mode::Added => print!("+ "),
    //         Mode::Multiplied => print!("* "),
    //     }
    // }
    // print!("\n");
    let operand_count = operand_array.len();
    let problem_count = operand_array
        .get(0)
        .expect("there should be some operand list")
        .len();
    assert_eq!(
        problem_count,
        operators.len(),
        "operand_count: {} does not match {problem_count}",
        operators.len()
    );
    for (i, ops) in operand_array.iter().enumerate() {
        assert_eq!(
            problem_count,
            ops.len(),
            "operand_array {i} does not match {problem_count}"
        );
    }
    let mut results: Vec<i64> = Vec::with_capacity(problem_count);
    for i in 0..problem_count {
        let mut operands: Vec<i64> = Vec::new();
        for op in 0..operand_count {
            operands.push(operand_array[op][i]);
        }
        // for op in &operands {
        //     print!("{op}");
        // }
        match operators[i] {
            Mode::Added => {
                //print!("+");
                results.push(operands.iter().sum())
            }
            Mode::Multiplied => {
                //print!("*");
                results.push(operands.iter().product())
            }
        }
        //print!("\n");
    }
    results.iter().sum::<i64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut ascii_grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut ascii_row: Vec<char> = Vec::new();
        for char in line.chars() {
            ascii_row.push(char);
        }
        ascii_grid.push(ascii_row);
    }
    let mut operators: Vec<Mode> = Vec::new();
    let mut operator_pos: Vec<usize> = Vec::new();

    // last row should contain operators
    for (pos, ch) in ascii_grid
        .last()
        .expect("there should be a last row")
        .iter()
        .enumerate()
    {
        match *ch {
            '+' => {
                operator_pos.push(pos);
                operators.push(Mode::Added);
            }
            '*' => {
                operator_pos.push(pos);
                operators.push(Mode::Multiplied);
            }
            _ => {}
        }
    }
    let max_digits = ascii_grid.len() - 1;

    // now that we have the operator position we can just scoop up the operands using those positions
    let mut operand_array: Vec<Vec<i64>> = Vec::new();
    for (left, right) in operator_pos.iter().tuple_windows() {
        let mut operandn: Vec<i64> = Vec::new();
        for col in *left..(*right - 1) {
            let mut textn: Vec<char> = Vec::new();
            for row in 0..max_digits {
                textn.push(ascii_grid[row][col]);
            }
            let text: String = textn.iter().filter(|x| x.is_digit(10)).collect();
            //println!("found number: {text}");
            operandn.push(text.parse().expect("should be a number"));
        }
        operand_array.push(operandn);
        //println!();
    }

    // and then we grab the last one
    let mut operandn: Vec<i64> = Vec::new();
    let left = *(operator_pos.last().expect("there must be a last"));
    let right = ascii_grid.last().expect("there must be a last").len();
    for col in left..right {
        let mut textn: Vec<char> = Vec::new();
        for row in 0..max_digits {
            textn.push(ascii_grid[row][col]);
        }
        let text: String = textn.iter().filter(|x| x.is_digit(10)).collect();
        //println!("found number: {text}");
        operandn.push(text.parse().expect("should be a number"));
    }
    operand_array.push(operandn);

    // run the operations
    let mut total: i64 = 0;
    for (problem, row) in operand_array.iter().enumerate() {
        let mut problem_operands: Vec<i64> = Vec::new();
        for col in row {
            problem_operands.push(*col);
        }
        match operators[problem] {
            Mode::Added => total += problem_operands.iter().sum::<i64>(),
            Mode::Multiplied => total += problem_operands.iter().product::<i64>(),
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "4277556");
        assert_eq!(process_part2(file), "3263827");
    }
}
