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
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "4277556");
    }
}
