use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type FullNetwork = HashMap<String, Vec<String>>;
type SubGroup = Vec<[String; 3]>;
type LargeSubGroup = HashSet<Vec<String>>;

fn subgroup_3<'a>(a: &'a str, b: &'a str, net: &FullNetwork) -> Vec<[String; 3]> {
    let mut res: Vec<[String; 3]> = Vec::new();
    //a and b are connected, check a and b's connections for anything that overlaps
    if let (Some(list_a), Some(list_b)) = (net.get(a), net.get(b)) {
        for node in list_a {
            if list_b.contains(node) {
                let mut node_group = [a.to_string(), b.to_string(), node.clone()];
                node_group.sort();
                res.push(node_group);
            }
        }
        res
    } else {
        res
    }
}

fn parse(input: &str) -> (FullNetwork, SubGroup) {
    let mut full_net: FullNetwork = HashMap::new();
    let mut groups: SubGroup = Vec::new();
    for line in input.lines() {
        let mut node_itr = line.split('-');
        let a = node_itr.next().expect("there should be a node").to_string();
        let b = node_itr.next().expect("there should be a node").to_string();
        full_net
            .entry(a.clone())
            .and_modify(|v| v.push(b.clone()))
            .or_insert(vec![b.clone()]);
        full_net
            .entry(b.clone())
            .and_modify(|v| v.push(a.clone()))
            .or_insert(vec![a.clone()]);
        for group in subgroup_3(&a, &b, &full_net) {
            if !groups.contains(&group) {
                groups.push(group);
            }
        }
    }
    (full_net, groups)
}

pub fn process_part1(input: &str) -> String {
    let (_, mut groups) = parse(input);
    groups.sort_by(|a, b| match a[0].cmp(&b[0]) {
        std::cmp::Ordering::Equal => match a[1].cmp(&b[1]) {
            std::cmp::Ordering::Equal => a[2].cmp(&b[2]),
            x => x,
        },
        x => x,
    });
    //println!("full_net: {full:?}");
    //println!("groups: {groups:?}");
    let mut total = 0;
    for group in groups {
        if group[0].starts_with('t') || group[1].starts_with('t') || group[2].starts_with('t') {
            total += 1;
        }
    }
    total.to_string()
}

fn super_get(input: &Vec<String>, net: &FullNetwork) -> Vec<(String, Vec<String>)> {
    let mut res = Vec::new();
    for node in input {
        if let Some(list) = net.get(node) {
            res.push((node.clone(), list.clone()));
        }
    }
    res
}

fn is_subgroup(list: &Vec<String>, net: &FullNetwork) -> bool {
    let big_list = super_get(list, net);
    for (node, adj_list) in big_list {
        for possible in list.iter() {
            if node == *possible || adj_list.contains(possible) {
                continue;
            } else {
                return false;
            }
        }
    }
    true
}

fn overlap_nodes(a: &String, b: &String, net: &FullNetwork) -> Vec<String> {
    let mut overlap: Vec<String> = Vec::new();
    if let (Some(list_a), Some(list_b)) = (net.get(a), net.get(b)) {
        for node in list_a {
            if list_b.contains(node) {
                overlap.push(node.clone());
            }
        }
    }
    overlap.push(a.clone());
    overlap.push(b.clone());
    overlap
}

fn parse2(input: &str) -> (FullNetwork, LargeSubGroup) {
    let mut full_net: FullNetwork = HashMap::new();
    let mut groups: LargeSubGroup = HashSet::new();
    for line in input.lines() {
        //println!("on line number: {line_num}");
        let mut node_itr = line.split('-');
        let a = node_itr.next().expect("there should be a node").to_string();
        let b = node_itr.next().expect("there should be a node").to_string();
        full_net
            .entry(a.clone())
            .and_modify(|v| v.push(b.clone()))
            .or_insert(vec![b.clone()]);
        full_net
            .entry(b.clone())
            .and_modify(|v| v.push(a.clone()))
            .or_insert(vec![a.clone()]);
        let mut overlap = overlap_nodes(&a, &b, &full_net);
        overlap.sort();
        if !groups.contains(&overlap) && is_subgroup(&overlap, &full_net) {
            //println!("found new subgroup: {group:?}");
            groups.insert(overlap.clone());
        }
        for i in 3..overlap.len() {
            for mut group in overlap.iter().cloned().combinations(i).unique() {
                group.sort();
                if !groups.contains(&group) && is_subgroup(&group, &full_net) {
                    //println!("found new subgroup: {group:?}");
                    groups.insert(group.clone());
                }
            }
        }
    }
    (full_net, groups)
}

pub fn process_part2(input: &str) -> String {
    let (_, group_set) = parse2(input);
    let groups: Vec<Vec<String>> = group_set
        .into_iter()
        .sorted_by(|a, b| b.len().cmp(&a.len()))
        .collect();
    //let longest = groups.iter().map(|x| x.len()).max().unwrap();
    // for group in groups.iter().filter(|x| x.len() == longest) {
    //     println!("{}:{group:?}", group.len());
    // }
    // println!(
    //     "number of groups that are the same size of {longest} nodes: {}",
    //     groups.iter().filter(|x| x.len() == longest).count()
    // );
    groups[0].join(",")
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
