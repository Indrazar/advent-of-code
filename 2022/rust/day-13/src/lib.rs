#![feature(iter_intersperse)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
use std::{fmt::Display, iter::zip, cmp::Ordering};

struct Pair {
    left: Packet,
    right: Packet,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nleft: {}\nright: {}", self.left, self. right)
    }
}

#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
                Packet::List(list) => format!( "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), packet),
            tag("]"),
        )
        .map(|vec| Packet::List(vec)),
        nom::character::complete::u32
            .map(|num| Packet::Number(num)),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(
            |(p1, p2)| Pair {
                left: p1,
                right: p2,
            },
        ),
    )(input)
}


impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::List(l_list) => match other {
                Packet::List(r_list) => {
                    let l_len = l_list.len();
                    let r_len = r_list.len();
                    for (l, r) in zip(l_list, r_list) {
                        match l.cmp(r) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Equal => continue,
                        }
                    }
                    if l_len > r_len {
                        //println!("list vs list, left longer, WRONG");
                        return Ordering::Greater
                    }
                    else if r_len > l_len {
                        //println!("list vs list, right longer, done");
                        return Ordering::Less
                    }
                    else { return Ordering::Equal }
                },
                Packet::Number(r_val) => {
                    let l_len = l_list.len();
                    let r_len = 1;
                    match Packet::List((*l_list).clone()).cmp(&Packet::List(vec![Packet::Number(*r_val)])) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {
                            if l_len > r_len {
                                //println!("list vs value, list too long, WRONG");
                                return Ordering::Greater
                            } else if  l_len < r_len {
                                //println!("empty list vs value, done");
                                return Ordering::Less
                            }
                            else { return Ordering::Equal }
                        },
                    }
                },
            },
            Packet::Number(l_val) => match other {
                Packet::List(r_list) => {
                    match (Packet::List(vec![Packet::Number(*l_val)])).cmp(&Packet::List((*r_list).clone())) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => return Ordering::Less,
                    }
                },
                Packet::Number(r_val) => {
                    //println!("l_val: {l_val}, r_val: {r_val}");
                    if l_val < r_val {
                        //println!("left is smaller, done");
                        return Ordering::Less }
                    else if r_val < l_val {
                        //println!("right is larger, WRONG");
                        return Ordering::Greater
                    }
                    else { return Ordering::Equal }
                },
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

pub fn process_part1(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    let mut correct_count = 0;
    let mut pair_count = 1;
    //for pair in pair_list {
    //    print!("{}\n", pair);
    //}
    for pair in pair_list {
        match pair.left.cmp(&pair.right) {
            Ordering::Less => {
                //println!("{pair_count} is ordered correctly");
                correct_count += pair_count;
            },
            Ordering::Equal => {
                //println!("{pair_count} is ordered correctly");
                panic!("should not get here")
                //correct_count += pair_count;
            },
            Ordering::Greater => {
                //println!("{pair_count} is ordered incorrectly");
            },
        }
        pair_count += 1;
    }
    correct_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, pair_input) = pairs(input).unwrap();
    let mut pair_list: Vec<Packet> = Vec::new();
    for pair in pair_input {
        pair_list.push(pair.left);
        pair_list.push(pair.right);
    }
    pair_list.push(Packet::List(vec![Packet::List(vec![Packet::Number(2)])]));
    pair_list.push(Packet::List(vec![Packet::List(vec![Packet::Number(6)])]));
    pair_list.sort();
    let mut iter = pair_list.iter();
    let mut iter2 = pair_list.iter();
    let pos1 = iter.position(|x| *x == Packet::List(vec![Packet::List(vec![Packet::Number(2)])])).unwrap();
    let pos2 = iter2.position(|x| *x == Packet::List(vec![Packet::List(vec![Packet::Number(6)])])).unwrap();
    //for pair in pair_list {
    //    println!("{pair}");
    //}
    ((pos1+1) * (pos2+1)).to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "13");
        assert_eq!(process_part2(file.as_str()), "140");
    }
}
