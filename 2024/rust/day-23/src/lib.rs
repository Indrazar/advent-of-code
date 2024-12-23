use std::collections::HashMap;

type FullNetwork = HashMap<String, Vec<String>>;
type SubGroup = Vec<[String; 3]>;
type LargeSubGroup = Vec<Vec<String>>;

fn subgroup<'a>(a: &'a str, b: &'a str, net: &FullNetwork) -> Vec<[String; 3]> {
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
        for group in subgroup(&a, &b, &full_net) {
            if !groups.contains(&group) {
                groups.push(group);
            }
        }
    }
    (full_net, groups)
}

pub fn process_part1(input: &str) -> String {
    let (full, mut groups) = parse(input);
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
        if group[0].chars().next().unwrap() == 't'
            || group[1].chars().next().unwrap() == 't'
            || group[2].chars().next().unwrap() == 't'
        {
            total += 1;
        } else {
        }
    }
    total.to_string()
}

//fn parse2(input: &str) -> (FullNetwork, SubGroup)

pub fn process_part2(input: &str) -> String {
    "works".to_string()
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
