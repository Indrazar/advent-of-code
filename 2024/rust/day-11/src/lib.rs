use std::collections::HashMap;

use nom::{
    character::complete::{self, anychar},
    multi::{many1, many_till},
    IResult, Parser,
};

fn iterate_stones(input: Vec<u64>) -> Vec<u64> {
    let mut new: Vec<u64> = Vec::with_capacity(input.len() * 2);
    for stone in input.iter() {
        if *stone == 0 {
            new.push(1);
            continue;
        }
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let (first, last) = stone_str.split_at(stone_str.len() / 2);
            new.push(first.parse().expect("should be a number"));
            new.push(last.parse().expect("should be a number"));
            continue;
        } else {
            new.push((*stone) * 2024);
        }
    }
    new
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    many1(many_till(anychar, complete::u64).map(|(_discard, val)| val))(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, mut stones) = parse(input).expect("parse should succeed");
    //println!("{stones:?}");
    for _ in 0..25 {
        stones = iterate_stones(stones);
        //if i < 6 {
        //    println!("{stones:?}");
        //}
    }
    stones.len().to_string()
}

fn determine_trajectory(input: u64, generations: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if generations == 1 {
        // take current stone, iterate one generation, add to map
        let single_stone = vec![input];
        let stone_up_to_pair = iterate_stones(single_stone);
        match map.get(&(input, generations)) {
            Some(val) => *val,
            None => {
                map.insert((input, generations), stone_up_to_pair.len() as u64);
                stone_up_to_pair.len() as u64
            }
        }
    } else {
        //println!("requesting {input} after {} generations", generations);
        match map.get(&(input, generations)) {
            Some(val) => *val,
            None => {
                let single_stone = vec![input];
                let stone_up_to_pair = iterate_stones(single_stone);
                let mut sum: u64 = 0;
                for stone in stone_up_to_pair {
                    sum += determine_trajectory(stone, generations - 1, map);
                }
                //println!("found {sum} stones from a {input} stone after {generations} generations");
                map.insert((input, generations), sum);
                sum
            }
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let (_, stones) = parse(input).expect("parse should succeed");
    let mut trajectory_map: HashMap<(u64, u64), u64> = HashMap::new();
    //println!("{stones:?}");
    let mut sum: u64 = 0;
    for stone in stones {
        sum += determine_trajectory(stone, 75, &mut trajectory_map);
    }
    //println!("items in map: {}", trajectory_map.keys().len());
    sum.to_string()
}

pub fn process_part1_but_faster(input: &str) -> String {
    let (_, stones) = parse(input).expect("parse should succeed");
    let mut trajectory_map: HashMap<(u64, u64), u64> = HashMap::new();
    //println!("{stones:?}");
    let mut sum: u64 = 0;
    for stone in stones {
        sum += determine_trajectory(stone, 25, &mut trajectory_map);
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "55312");
    }
}
