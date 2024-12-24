use std::collections::{HashMap, VecDeque};

struct Gate {
    left: String,
    right: String,
    gate_type: GateType,
    output: String,
}

enum GateType {
    AND,
    OR,
    XOR,
}

fn parse_nodes(input: &str) -> HashMap<String, Option<u8>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut itr = line.split(":");
        let name: String = itr
            .next()
            .expect("node should have a name")
            .trim()
            .to_string();
        let val: u8 = itr
            .next()
            .expect("node should have a value")
            .trim()
            .parse()
            .expect("value should be a number");
        map.insert(name, Some(val));
    }
    map
}

fn parse_gates(input: &str) -> VecDeque<Gate> {
    let mut list = VecDeque::new();
    for line in input.lines() {
        let mut itr = line.split_whitespace();
        let left = itr
            .next()
            .expect("there should be a left on the gate")
            .to_string();
        let gate_type: GateType = match itr.next().expect("there should be a gate type") {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            unknown => panic!("unknown gate type: {unknown}"),
        };
        let right = itr
            .next()
            .expect("there should be a right on the gate")
            .to_string();
        if "->" != itr.next().expect("there should be an arrow") {
            panic!("missing arrow on parse line {line}");
        }
        let output: String = itr
            .next()
            .expect("there should be an output on the gate")
            .to_string();
        let gate: Gate = Gate {
            left,
            right,
            gate_type,
            output,
        };
        list.push_back(gate);
    }
    list
}

fn parse(input: &str) -> (HashMap<String, Option<u8>>, VecDeque<Gate>) {
    let mut itr = input.split("\n\n");
    let nodes = itr.next().expect("there should be a list of nodes");
    let gates = itr.next().expect("there should be a list of gates");
    let map = parse_nodes(nodes);
    let gate_list = parse_gates(gates);
    (map, gate_list)
}

pub fn process_part1(input: &str) -> String {
    let (mut map, mut gate_list) = parse(input);
    while !gate_list.is_empty() {
        let gate = gate_list.pop_front().unwrap();
        if let (Some(Some(a)), Some(Some(b))) = (map.get(&gate.left), map.get(&gate.right)) {
            // we can do this one now and discard it after we are done
            let result = match gate.gate_type {
                GateType::AND => a & b,
                GateType::OR => a | b,
                GateType::XOR => a ^ b,
            };
            map.insert(gate.output, Some(result));
        } else {
            // need to do this one later, we are missing an input
            gate_list.push_back(gate);
        }
    }
    let mut znodes: Vec<(String, u8)> = map
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| (k.clone(), v.unwrap()))
        .collect();
    znodes.sort_by(|(a, _), (b, _)| a.cmp(b));
    let result = znodes.iter().enumerate().fold(0u64, |acc, (i, (_, val))| {
        acc + (*val as u64 * 2u64.pow(i as u32))
    });
    result.to_string()
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
        assert_eq!(process_part1(file), "4");
        let file2 = include_str!("../test-input-2.txt");
        assert_eq!(process_part1(file2), "2024");
    }
}
