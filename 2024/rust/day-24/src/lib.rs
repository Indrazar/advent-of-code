use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    left: String,
    right: String,
    output: String,
    gate_type: GateType,
}

#[derive(Debug, Clone)]
struct SignalData {
    input_into: Vec<Gate>,
    output_from: Vec<Gate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GateType {
    AND,
    OR,
    XOR,
}

fn parse_nodes(input: &str) -> HashMap<String, u8> {
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
        map.insert(name, val);
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

fn parse(input: &str) -> (HashMap<String, u8>, VecDeque<Gate>) {
    let mut itr = input.split("\n\n");
    let nodes = itr.next().expect("there should be a list of nodes");
    let gates = itr.next().expect("there should be a list of gates");
    let map = parse_nodes(nodes);
    let gate_list = parse_gates(gates);
    (map, gate_list)
}

fn resolve_network(map: &HashMap<String, u8>, gates: &VecDeque<Gate>) -> u64 {
    let mut map = map.clone();
    let mut gate_list = gates.clone();
    while !gate_list.is_empty() {
        let gate = gate_list.pop_front().unwrap();
        if let (Some(a), Some(b)) = (map.get(&gate.left), map.get(&gate.right)) {
            // we can do this one now and discard it after we are done
            let result = match gate.gate_type {
                GateType::AND => a & b,
                GateType::OR => a | b,
                GateType::XOR => a ^ b,
            };
            map.insert(gate.output, result);
        } else {
            // need to do this one later, we are missing an input
            gate_list.push_back(gate);
        }
    }
    let mut znodes: Vec<(String, u8)> = map
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    znodes.sort_by(|(a, _), (b, _)| a.cmp(b));
    let result = znodes.iter().enumerate().fold(0u64, |acc, (i, (_, val))| {
        acc + (*val as u64 * 2u64.pow(i as u32))
    });
    result
}

pub fn process_part1(input: &str) -> String {
    let (map, gate_list) = parse(input);
    resolve_network(&map, &gate_list).to_string()
}

fn is_invalid_input_xor_gate(gate: &Gate) -> bool {
    (gate.left.starts_with("x") && !gate.right.starts_with("y"))
        || (gate.left.starts_with("y") && !gate.right.starts_with("x"))
}

fn invalid_input_xor(gate_list: &VecDeque<Gate>) -> Vec<Gate> {
    gate_list
        .iter()
        .filter(|g| is_invalid_input_xor_gate(g))
        .cloned()
        .collect()
}

fn output_bit_count(map: &HashMap<String, u8>) -> usize {
    let mut highest = 0;
    for k in map.keys() {
        if let Some(x) = k.strip_prefix("x") {
            let check: usize = x.parse().expect("there should be a number");
            if check > highest {
                highest = check;
            }
        }
    }
    highest + 1
}

fn generate_signal_map(gate_list: &VecDeque<Gate>) -> HashMap<String, SignalData> {
    let mut map = HashMap::new();
    for gate in gate_list {
        // left
        map.entry(gate.left.clone())
            .and_modify(|x: &mut SignalData| x.input_into.push(gate.clone()))
            .or_insert(SignalData {
                input_into: vec![gate.clone()],
                output_from: vec![],
            });
        // right
        map.entry(gate.right.clone())
            .and_modify(|x: &mut SignalData| x.input_into.push(gate.clone()))
            .or_insert(SignalData {
                input_into: vec![gate.clone()],
                output_from: vec![],
            });
        // output
        map.entry(gate.output.clone())
            .and_modify(|x: &mut SignalData| x.output_from.push(gate.clone()))
            .or_insert(SignalData {
                input_into: vec![],
                output_from: vec![gate.clone()],
            });
    }
    map
}

fn overlap_inputs(map: &HashMap<String, SignalData>, a: &String, b: &String) -> Vec<Gate> {
    if let (Some(a_d), Some(b_d)) = (map.get(a), map.get(b)) {
        let mut res = Vec::new();
        for gate_a in a_d.input_into.iter() {
            for gate_b in b_d.input_into.iter() {
                if gate_a == gate_b {
                    res.push(gate_a.clone());
                }
            }
        }
        res
    } else {
        vec![]
    }
}

fn count_or(input: &Vec<Gate>) -> u64 {
    let mut res = 0;
    for gate in input {
        if gate.gate_type == GateType::OR {
            res += 1;
        }
    }
    res
}

fn count_and(input: &Vec<Gate>) -> u64 {
    let mut res = 0;
    for gate in input {
        if gate.gate_type == GateType::AND {
            res += 1;
        }
    }
    res
}

fn count_xor(input: &Vec<Gate>) -> u64 {
    let mut res = 0;
    for gate in input {
        if gate.gate_type == GateType::XOR {
            res += 1;
        }
    }
    res
}

pub fn process_part2(input: &str) -> String {
    let (input_map, gate_list) = parse(input);
    //let mut intermediate_signals = list_of_intermediate_nodes(&gate_list);
    let bit_length = output_bit_count(&input_map);
    let signal_map = generate_signal_map(&gate_list);
    // start at X + Y
    //                            XOR -> Z
    // one branch should go XOR x
    //                            AND -> ETC
    //
    //                                    AND -> ETC
    // other branch should go AND -> OR x
    //                                    XOR -> Z
    // for this method to work we must have no messed up X+Y input layer gates
    let invalid_xor = invalid_input_xor(&gate_list);
    if !invalid_xor.is_empty() {
        panic!("cannot continue with solution");
    }
    let mut invalid_signal: Vec<String> = Vec::new();
    let mut visited_gates: Vec<Gate> = Vec::new();
    for i in 0..bit_length {
        let x_signal_name: String = format!("x{i:02}");
        let y_signal_name: String = format!("y{i:02}");
        let overlap = overlap_inputs(&signal_map, &x_signal_name, &y_signal_name);
        for gate in overlap.iter() {
            if !visited_gates.contains(gate) {
                visited_gates.push(gate.clone());
            }
        }
        //println!("overlap: {overlap:?}");
        if overlap.len() != 2 || count_and(&overlap) != 1 || count_xor(&overlap) != 1 {
            println!("invalid overlap: {overlap:?}")
        } else {
            let xor_signal = overlap
                .iter()
                .find(|x| x.gate_type == GateType::XOR)
                .expect("there is exactly one XOR")
                .output
                .clone();
            let and_signal = overlap
                .iter()
                .find(|x| x.gate_type == GateType::AND)
                .expect("there is exactly one XOR")
                .output
                .clone();
            // do xor first
            if xor_signal.starts_with("z") {
                // done
            } else {
                //keep going
                let xor_branch = signal_map
                    .get(&xor_signal)
                    .expect("there should be signal data")
                    .input_into
                    .clone();
                // mark gates visited
                for gate in xor_branch.iter() {
                    if !visited_gates.contains(gate) {
                        visited_gates.push(gate.clone())
                    }
                }
                if xor_branch.len() != 2
                    || count_and(&xor_branch) != 1
                    || count_xor(&xor_branch) != 1
                {
                    if !invalid_signal.contains(&xor_signal) {
                        invalid_signal.push(xor_signal.clone());
                    }
                    // println!(
                    //     "{xor_signal}: {i}, should be exactly 1 xor and 1 and: xor_branch: {xor_branch:?}"
                    // );
                } else {
                    // the XOR gate should have a Z output
                    //println!("gates: {xor_branch:?}");
                    let xor_xor_signal = xor_branch
                        .iter()
                        .find(|x| x.gate_type == GateType::XOR)
                        .expect("there is exactly one XOR")
                        .output
                        .clone();
                    if !xor_xor_signal.starts_with("z") {
                        if !invalid_signal.contains(&xor_xor_signal) {
                            invalid_signal.push(xor_xor_signal.clone());
                        }
                        // println!(
                        //     "{i}: invalid output of xor->xor (should be zxx): {xor_xor_signal}"
                        // );
                        let z_string = format!("z{i:02}");
                        // println!(
                        //     "invalid_signal contains z_string: {z_string}? {}",
                        //     invalid_signal.contains(&z_string)
                        // );
                        if !invalid_signal.contains(&z_string) {
                            invalid_signal.push(z_string);
                        }
                    }
                    // the AND gate should not
                    let xor_and_signal = xor_branch
                        .iter()
                        .find(|x| x.gate_type == GateType::AND)
                        .expect("there is exactly one XOR")
                        .output
                        .clone();
                    if xor_and_signal.starts_with("z") {
                        // println!(
                        //     "{i}: invalid output of xor->and (shouldn't be zxx): {xor_and_signal}"
                        // );
                        if !invalid_signal.contains(&xor_and_signal) {
                            invalid_signal.push(xor_and_signal);
                        }
                    }
                }
            }
            // now do and branch
            let and_branch = signal_map
                .get(&and_signal)
                .expect("there should be signal data")
                .input_into
                .clone();
            //mark gates visited
            for gate in and_branch.iter() {
                if !visited_gates.contains(gate) {
                    visited_gates.push(gate.clone());
                }
            }
            if and_branch.len() != 1 || count_or(&and_branch) != 1 {
                if and_branch.len() == 2
                    && count_xor(&and_branch) == 1
                    && count_and(&and_branch) == 1
                    && and_branch
                        .iter()
                        .find(|x| x.gate_type == GateType::XOR)
                        .unwrap()
                        .output
                        .starts_with("z01")
                {
                    // do nothing, it's correct (special z01 case)
                } else {
                    // it's wrong
                    if !invalid_signal.contains(&and_signal) {
                        invalid_signal.push(and_signal.clone());
                    }
                    // println!(
                    //     "{i}: {and_signal}: gates: {}, AND: {}, XOR: {}, invalid and_branch: {and_branch:?}",
                    //     and_branch.len(),
                    //     count_and(&and_branch),
                    //     count_xor(&and_branch)
                    // )
                }
            }

            //println!("and_signal: {and_signal}; gates: {and_branch:?}")
        }
    }
    //unvisited gates
    // let mut unvisited_gates: Vec<Gate> = Vec::new();
    // for gate in gate_list.iter() {
    //     if !visited_gates.contains(gate) {
    //         unvisited_gates.push(gate.clone());
    //     }
    // }
    // println!("unvisited gates: {:?}", unvisited_gates);
    invalid_signal.sort();
    //println!("invalid signals: {}", invalid_signal.join(","));
    invalid_signal.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "4");
    }
}
