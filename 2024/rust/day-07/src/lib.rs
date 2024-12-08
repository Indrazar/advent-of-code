#[derive(Debug, Clone)]
struct LineData {
    target: i64,
    inputs: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug, Clone)]
struct LineOperator {
    state: Vec<Operators>,
}

impl LineOperator {
    fn new(length: usize) -> LineOperator {
        LineOperator {
            state: vec![Operators::Add; length - 1],
        }
    }
    fn from(input: &LineData) -> LineOperator {
        let length = input.inputs.len() - 1;
        LineOperator {
            state: vec![Operators::Add; length],
        }
    }
    fn inc_state(&mut self) {
        for i in 0..self.state.len() {
            match self.state[i] {
                // no carry
                Operators::Add => {
                    self.state[i] = Operators::Multiply;
                    break;
                }
                // carry
                Operators::Multiply => {
                    self.state[i] = Operators::Add;
                }
                Operators::Concatenate => {
                    panic!("part 1 does not support concatenate")
                }
            }
        }
    }
    fn inc_state2(&mut self) {
        for i in 0..self.state.len() {
            match self.state[i] {
                // no carry
                Operators::Add => {
                    self.state[i] = Operators::Multiply;
                    break;
                }
                // no carry
                Operators::Multiply => {
                    self.state[i] = Operators::Concatenate;
                    break;
                }
                // carry
                Operators::Concatenate => {
                    self.state[i] = Operators::Add;
                }
            }
        }
    }
    fn test_state(&self, input: &LineData) -> bool {
        if input.inputs.is_empty() {
            panic!("input length is too small");
        }
        let target = input.target;
        let mut acc = input.inputs[0];
        for i in 1..input.inputs.len() {
            match self.state[i - 1] {
                Operators::Add => acc += input.inputs[i],
                Operators::Multiply => acc *= input.inputs[i],
                Operators::Concatenate => {
                    panic!("part 1 does not support concatenate")
                }
            }
        }
        acc == target
    }
    fn test_state2(&self, input: &LineData) -> bool {
        if input.inputs.is_empty() {
            panic!("input length is too small");
        }
        let target = input.target;
        let mut acc = input.inputs[0];
        for i in 1..input.inputs.len() {
            match self.state[i - 1] {
                Operators::Add => acc += input.inputs[i],
                Operators::Multiply => acc *= input.inputs[i],
                Operators::Concatenate => {
                    acc = (format!("{acc}{}", input.inputs[i]))
                        .parse()
                        .expect("value should still be a number")
                }
            }
        }
        acc == target
    }
}

fn parse_line(input: &str) -> LineData {
    let cleaved: Vec<&str> = input.split(':').collect();
    let mut inputs: Vec<i64> = Vec::new();
    let target: i64 = cleaved[0].parse().expect("parse should succeed");
    let input_str: Vec<&str> = cleaved[1].split(' ').collect();
    for i in input_str.iter().skip(1) {
        inputs.push(i.parse().expect("parse should succeed"));
    }
    //println!("new line parsed target: {target}, inputs: {inputs:?}");
    LineData { target, inputs }
}

fn solve_line_data(input: LineData) -> i64 {
    let mut line_operator: LineOperator = LineOperator::from(&input);
    let mut iterations: usize = 0;
    let bits: u32 = line_operator
        .state
        .len()
        .try_into()
        .expect("number of operators is too long");
    let max_iterations: usize = usize::pow(2, bits);
    let mut correct_equations: i64 = 0;
    while iterations < max_iterations {
        //println!(
        //    "using target: {}, inputs, {:?} testing {:?}",
        //    input.target, input.inputs, line_operator.state
        //);
        if line_operator.test_state(&input) {
            //println!("was correct");
            correct_equations += 1
        }
        line_operator.inc_state();
        iterations += 1;
    }
    if correct_equations > 0 {
        input.target
    } else {
        0
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|l| -> i64 { solve_line_data(parse_line(l)) })
        .sum::<i64>()
        .to_string()
}

fn solve_line_data2(input: LineData) -> i64 {
    let mut line_operator: LineOperator = LineOperator::from(&input);
    let mut iterations: usize = 0;
    let bits: u32 = line_operator
        .state
        .len()
        .try_into()
        .expect("number of operators is too long");
    let max_iterations: usize = usize::pow(3, bits);
    let mut correct_equations: i64 = 0;
    while iterations < max_iterations {
        //println!(
        //    "using target: {}, inputs, {:?} testing {:?}",
        //    input.target, input.inputs, line_operator.state
        //);
        if line_operator.test_state2(&input) {
            //println!("was correct");
            correct_equations += 1
        }
        line_operator.inc_state2();
        iterations += 1;
    }
    if correct_equations > 0 {
        input.target
    } else {
        0
    }
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|l| -> i64 { solve_line_data2(parse_line(l)) })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "3749");
        assert_eq!(process_part2(file), "11387");
    }
}
