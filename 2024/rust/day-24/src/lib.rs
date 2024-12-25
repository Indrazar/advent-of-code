use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::zip,
};

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    right: String,
    gate_type: GateType,
    output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GateType {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GateFunction {
    OutputSum(u64),
    InputSum(u64),
    CarryFlag(u64),
}

impl GateFunction {
    fn inner(&self) -> u64 {
        match self {
            GateFunction::OutputSum(i) => *i,
            GateFunction::InputSum(i) => *i,
            GateFunction::CarryFlag(i) => *i,
        }
    }
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
    // let intermediate_nodes: Vec<(String, u8)> = map
    //     .iter()
    //     .filter(|(k, _)| !k.starts_with("x") && !k.starts_with("y") && !k.starts_with("z"))
    //     .map(|(k, v)| (k.clone(), *v))
    //     .collect();
    // println!("there are {} intermediate nodes", intermediate_nodes.len());
    result
}

pub fn process_part1(input: &str) -> String {
    let (map, gate_list) = parse(input);
    resolve_network(&map, &gate_list).to_string()
}

fn resolve_network_in_place(map: &mut HashMap<String, u8>, gates: &mut VecDeque<Gate>) -> u64 {
    while !gates.is_empty() {
        let gate = gates.pop_front().unwrap();
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
            gates.push_back(gate);
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

fn list_of_intermediate_nodes(gates: &VecDeque<Gate>) -> Vec<String> {
    let mut map: HashSet<String> = HashSet::new();
    for gate in gates {
        for node in [&gate.left, &gate.right, &gate.output].iter() {
            if !node.starts_with("x") && !node.starts_with("y") && !node.starts_with("z") {
                map.insert((**node).clone());
            }
        }
    }
    let mut list: Vec<String> = map.iter().cloned().collect();
    list.sort();
    list
}

fn is_invalid_output_gate(gate: &Gate, max_bits: usize) -> bool {
    let max_bits_z = format!("z{max_bits}");
    (gate.output.starts_with("z") && gate.gate_type != GateType::XOR)
        && !(gate.output == max_bits_z && gate.gate_type == GateType::OR)
}

fn is_invalid_xor_gate(gate: &Gate) -> bool {
    (gate.left.starts_with("x") && !gate.right.starts_with("y"))
        || (gate.left.starts_with("y") && !gate.right.starts_with("x"))
}

fn invalid_xor(gate_list: &VecDeque<Gate>) -> Vec<(String, String, String)> {
    gate_list
        .iter()
        .filter(|g| is_invalid_xor_gate(g))
        .map(|g| (g.left.clone(), g.right.clone(), g.output.clone()))
        .collect()
}

fn invalid_output_xor(
    gate_list: &VecDeque<Gate>,
    max_bits: usize,
) -> Vec<(String, String, String, GateType, u64)> {
    let mut list: Vec<(String, String, String, GateType, u64)> = gate_list
        .iter()
        .filter(|g| is_invalid_output_gate(g, max_bits))
        .map(|g| {
            (
                g.output.clone(),
                g.left.clone(),
                g.right.clone(),
                g.gate_type,
                g.output[1..].parse().expect("there should be a number"),
            )
        })
        .collect();
    list.sort_by(|(_, _, _, _, a), (_, _, _, _, b)| a.cmp(b));
    list
}

fn is_first_layer_xor(gate: &Gate) -> bool {
    (gate.left.starts_with("x") || gate.left.starts_with("y"))
        && (gate.right.starts_with("x") || gate.right.starts_with("y"))
        && gate.gate_type == GateType::XOR
}

fn first_layer_xor(gate_list: &VecDeque<Gate>) -> Vec<(String, u64)> {
    let mut list: Vec<(String, u64)> = gate_list
        .iter()
        .filter(|g| is_first_layer_xor(*g))
        .map(|g| {
            if g.left[1..] != g.right[1..] {
                panic!("left != right: {}, {}", g.left, g.right)
            }
            (
                g.output.clone(),
                g.left[1..].parse().expect("should be a number"),
            )
        })
        .collect();
    list.sort_by(|(_, a), (_, b)| a.cmp(b));
    list
}

fn output_bit_count(map: &HashMap<String, u8>) -> usize {
    let mut highest = 0;
    for (k, _) in map {
        if k.starts_with("x") {
            let check: usize = k[1..].parse().expect("there should be a number");
            if check > highest {
                highest = check;
            }
        }
    }
    highest + 1
}

fn is_output_layer_xor(gate: &Gate) -> bool {
    gate.output.starts_with("z") && gate.gate_type == GateType::XOR
}

fn output_layer_xor(gate_list: &VecDeque<Gate>) -> Vec<(String, String, u64)> {
    let mut list: Vec<(String, String, u64)> = gate_list
        .iter()
        .filter(|g| !is_first_layer_xor(*g) && is_output_layer_xor(*g))
        .map(|g| {
            (
                g.left.clone(),
                g.right.clone(),
                g.output[1..].parse().expect("there should be a number"),
            )
        })
        .collect();
    list.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));
    list
}

fn intermediate_layer_xor(
    gate_list: &VecDeque<Gate>,
    max_bits: usize,
) -> Vec<(String, String, String)> {
    gate_list
        .iter()
        .filter(|g| {
            g.gate_type == GateType::XOR
                && !is_first_layer_xor(g)
                && !is_invalid_output_gate(g, max_bits)
                && !is_output_layer_xor(g)
        })
        .map(|g| (g.left.clone(), g.right.clone(), g.output.clone()))
        .collect()
}

pub fn process_part2(input: &str) -> String {
    let (input_map, gate_list) = parse(input);
    let mut intermediate_signals = list_of_intermediate_nodes(&gate_list);
    let bit_length = output_bit_count(&input_map);
    println!("output has {bit_length} bits");
    println!("there are {} gates", gate_list.len());
    println!(
        "there are {} XOR gates",
        gate_list
            .iter()
            .filter(|x| x.gate_type == GateType::XOR)
            .count()
    );
    println!(
        "there are {} AND gates",
        gate_list
            .iter()
            .filter(|x| x.gate_type == GateType::AND)
            .count()
    );
    println!(
        "there are {} OR gates",
        gate_list
            .iter()
            .filter(|x| x.gate_type == GateType::OR)
            .count()
    );
    // find all invalid XOR input layer
    let invalid_xor = invalid_xor(&gate_list);
    println!(
        "there are {} invalid xor input layer gates: {invalid_xor:?}",
        invalid_xor.len()
    );
    // collect list of first layer XOR outputs
    let first_layer_xor: Vec<(String, u64)> = first_layer_xor(&gate_list);
    if !first_layer_xor.contains(&(String::from("z00"), 0u64)) {
        panic!("could not find z00");
    }
    println!(
        "there are {} first layer gates: {first_layer_xor:?}",
        first_layer_xor.len()
    );
    // get invalid output layer XORs
    let invalid_output_xor: Vec<(String, String, String, GateType, u64)> =
        invalid_output_xor(&gate_list, bit_length);
    println!(
        "there are {} invalid output gates: {invalid_output_xor:?}",
        invalid_output_xor.len()
    );
    // collect list of last layer XOR outputs
    let mut output_layer_xor: Vec<(String, String, u64)> = output_layer_xor(&gate_list);
    println!(
        "there are {} last layer gates: {output_layer_xor:?}",
        output_layer_xor.len()
    );
    let mut resolved_signals: Vec<(String, GateFunction)> = Vec::new();
    let mut known_wrong_signals: Vec<(String, GateFunction)> = Vec::new();
    let mut unresolved_signals: Vec<(String, GateFunction)> = Vec::new();
    let mut last_adj = 1;
    let mut _incorrect_found = 0;
    for i in 1..bit_length {
        let last = i - last_adj;
        if output_layer_xor[last].2 != i as u64 {
            // must be one of the invalid gates
            known_wrong_signals.push((
                first_layer_xor[i].0.clone(),
                GateFunction::InputSum(i as u64),
            ));
            _incorrect_found += 1;
            last_adj += 1;
            //invalid_output_xor[0].
        } else {
            //if output_layer_xor[last].2 == i as u64 {
            // we found a matched signal pair
            if output_layer_xor[last].0 == first_layer_xor[i].0 {
                // left side matches
                // resolve left
                resolved_signals.push((
                    first_layer_xor[i].0.clone(),
                    GateFunction::InputSum(i as u64),
                ));
                // right is carry
                unresolved_signals.push((
                    output_layer_xor[last].1.clone(),
                    GateFunction::CarryFlag(i as u64),
                ));
            } else if output_layer_xor[last].1 == first_layer_xor[i].0 {
                // right side matches
                // reslove right
                resolved_signals.push((
                    first_layer_xor[i].0.clone(),
                    GateFunction::InputSum(i as u64),
                ));
                // left is carry
                unresolved_signals.push((
                    output_layer_xor[last].0.clone(),
                    GateFunction::CarryFlag(i as u64),
                ));
            }
        }
    }
    println!("{} intermediate_signals", intermediate_signals.len());
    println!("{} resolved_signals", resolved_signals.len());
    println!(
        "{} known_wrong_signals: {known_wrong_signals:?}",
        known_wrong_signals.len()
    );
    println!(
        "{} unresolved signals: {unresolved_signals:?}",
        unresolved_signals.len()
    );
    let mut swap_pair: Vec<(String, String)> = Vec::new();
    for wrong in invalid_output_xor.iter() {
        for wrong2 in known_wrong_signals.iter() {
            let inner = wrong2.1.inner();
            if wrong.4 == inner {
                swap_pair.push((wrong2.0.clone(), wrong.0.clone()));
                break;
            }
        }
    }
    println!("{} known swaps: {swap_pair:?}", swap_pair.len());
    let mut remaining_wrong_signals: Vec<(String, String, String, GateType, u64)> = Vec::new();
    for wrong in invalid_output_xor.iter() {
        let mut found = false;
        for pair in swap_pair.iter() {
            if pair.0 == wrong.0 || pair.1 == wrong.0 {
                found = true;
                break;
            }
        }
        if !found {
            remaining_wrong_signals.push(wrong.clone());
        }
    }
    println!(
        "{} remaining wrong gates: {remaining_wrong_signals:?}",
        remaining_wrong_signals.len()
    );
    for (left, right, idx) in output_layer_xor.iter() {
        let mut and_gate_found = false;
        for gate in gate_list.iter() {
            if ((gate.left == *left && gate.right == *right)
                || (gate.right == *left && gate.left == *right))
                && gate.gate_type == GateType::AND
            {
                and_gate_found = true;
                resolved_signals.push((gate.output.clone(), GateFunction::CarryFlag(*idx)));
                break;
            }
        }
        if !and_gate_found {
            println!("missing AND gate for {left} and {right} for index: {idx}");
        }
    }
    resolved_signals.sort_by(|a, b| a.1.inner().cmp(&b.1.inner()));
    println!(
        "{} resolved signals: {resolved_signals:?}",
        resolved_signals.len()
    );
    for i in 2..bit_length {
        let mut found_input_sum = false;
        let mut found_carry_flag = false;
        for signal in resolved_signals.iter() {
            if signal.1.inner() == i as u64 {
                if signal.1 == GateFunction::InputSum(i as u64) {
                    found_input_sum = true;
                } else if signal.1 == GateFunction::CarryFlag(i as u64) {
                    found_carry_flag = true;
                }
            }
        }
        if !found_input_sum {
            println!("couldn't find an input_sum for {i}");
        }
        if !found_carry_flag {
            println!("couldn't find a carry_flag for {i}")
        }
    }
    let small_resolved = resolved_signals
        .iter()
        .map(|x| x.0.clone())
        .collect::<Vec<String>>();

    let unpaired_xor_gates: Vec<Gate> = gate_list
        .iter()
        .filter(|x| {
            x.gate_type == GateType::XOR
                && !x.left.starts_with("x")
                && !x.right.starts_with("x")
                && !x.left.starts_with("y")
                && !x.right.starts_with("y")
                && !(small_resolved.contains(&x.right))
                && !(small_resolved.contains(&x.left))
        })
        .cloned()
        .collect();
    println!(
        "{} unpaired xor gates: {unpaired_xor_gates:?}",
        unpaired_xor_gates.len()
    );
    // for (signal, gf) in unresolved_signals {
    //     match gf {
    //         GateFunction::OutputSum(_) => todo!(),
    //         GateFunction::InputSum(_) => todo!(),
    //         GateFunction::CarryFlag(idx) => {
    //             let mut and_found = false;
    //             for gate in gate_list.iter() {
    //                 if gate.gate_type == GateType::AND && gate.output == signal {
    //                     and_found = true;
    //                 }
    //             }
    //             if !and_found {
    //                 println!("wrong signal {signal} {idx}")
    //             }
    //         }
    //     }
    // }

    //println!("");
    // also check =bit_length, but first_layer won't be included, only carries
    // for ((signal, a_bit), (l, r, o_bit)) in
    //     zip(first_layer_xor[1..].iter(), output_layer_xor.iter())
    // {
    //     if *a_bit != *o_bit {
    //         panic!("unknown missing bit: ({signal}, {a_bit}); ({l}, {r}, {o_bit})");
    //     }
    // }
    // remaning nodes:
    // let mut remaining_nodes: Vec<Gate> = gate_list
    //     .iter()
    //     .filter(|g| !is_first_layer_xor(g) && !is_output_layer_xor(g))
    //     .cloned()
    //     .collect();
    // println!(
    //     "there are {} remaning unpaired gates: {remaining_nodes:?}",
    //     remaining_nodes.len()
    // );
    let mut xnodes: Vec<(String, u8)> = input_map
        .iter()
        .filter(|(k, _)| k.starts_with("x"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    xnodes.sort_by(|(a, _), (b, _)| a.cmp(b));
    // let xresult = xnodes.iter().enumerate().fold(0u64, |acc, (i, (_, val))| {
    //     acc + (*val as u64 * 2u64.pow(i as u32))
    // });
    let mut ynodes: Vec<(String, u8)> = input_map
        .iter()
        .filter(|(k, _)| k.starts_with("y"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    ynodes.sort_by(|(a, _), (b, _)| a.cmp(b));
    // let yresult = ynodes.iter().enumerate().fold(0u64, |acc, (i, (_, val))| {
    //     acc + (*val as u64 * 2u64.pow(i as u32))
    // });
    if xnodes.len() != ynodes.len() {
        panic!("x and y input node counts do not match");
    }

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
    #[test]
    fn test_input2() {
        let file2 = include_str!("../test-input-2.txt");
        assert_eq!(process_part2(file2), "aaa,aoc,bbb,ccc,eee,ooo,z24,z99")
    }
}
