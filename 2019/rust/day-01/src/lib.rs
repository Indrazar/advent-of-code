pub fn process_part1(input: &str) -> String {
    let mut modules: Vec<i64> = Vec::new();
    for line in input.lines() {
        modules.push(line.parse().expect("parse should succeed"));
    }
    let mut sum = 0;
    for module in modules {
        sum += (module / 3) - 2;
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut modules: Vec<i64> = Vec::new();
    for line in input.lines() {
        modules.push(line.parse().expect("parse should succeed"));
    }
    let mut sum = 0;
    for module in modules {
        let mut additonal_fuel = (module / 3) - 2;
        while additonal_fuel > 0 {
            sum += additonal_fuel;
            additonal_fuel = (additonal_fuel / 3) - 2;
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "34241");
    }
}
