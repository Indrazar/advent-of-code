use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    multi::{many1, many_till},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RobotData {
    postion: Coord,
    velocity: Coord,
}

fn parse_robot(input: &str) -> IResult<&str, RobotData> {
    let (input, pos) = preceded(
        tag("p="),
        separated_pair(complete::i64, tag(","), complete::i64),
    )(input)?;
    let (input, vel) = preceded(
        tag(" v="),
        separated_pair(complete::i64, tag(","), complete::i64),
    )(input)?;
    let (input, _) = alt((line_ending, tag("")))(input)?;
    Ok((
        input,
        RobotData {
            postion: Coord { x: pos.0, y: pos.1 },
            velocity: Coord { x: vel.0, y: vel.1 },
        },
    ))
}

fn parse(input: &str) -> Vec<RobotData> {
    let (_, data) = many1(many_till(anychar, parse_robot).map(|(_discard, val)| val))(input)
        .expect("parse should succeed");
    data
}

fn print_map(data: HashMap<Coord, i64>, width: i64, height: i64) {
    let mut asterisks: Vec<(Coord, i64)> = Vec::new();
    println!("current map:");
    for y in 0..height {
        for x in 0..width {
            match data.get(&Coord { x, y }) {
                Some(val) => {
                    if *val > 9 && *val < 0 {
                        asterisks.push((Coord { x, y }, *val));
                        print!("*");
                    } else {
                        print!("{val}");
                    }
                }
                None => {
                    print!(".");
                }
            }
        }
        println!()
    }
    for asterisk in asterisks {
        println!("*: {asterisk:?}");
    }
}

fn process_map(width: i64, height: i64, data: Vec<RobotData>, iterations: i64) -> String {
    let mut new_data = data.clone();
    for _ in 0..iterations {
        for robot in new_data.iter_mut() {
            robot.postion.x += robot.velocity.x;
            robot.postion.y += robot.velocity.y;
            if robot.postion.x < 0 {
                robot.postion.x += width;
            } else if robot.postion.x >= width {
                robot.postion.x -= width;
            }
            if robot.postion.y < 0 {
                robot.postion.y += height;
            } else if robot.postion.y >= height {
                robot.postion.y -= height;
            }
        }
    }
    // build map of robots
    let mut robot_map: HashMap<Coord, i64> = HashMap::new();
    for robot in new_data {
        match robot_map.get_mut(&robot.postion) {
            Some(val) => {
                *val += 1;
            }
            None => {
                robot_map.insert(robot.postion, 1);
            }
        }
    }
    let robot_map = robot_map;
    let vertical_divider_inclusive = width % 2 == 0;
    let horizontal_divider_inclusive = height % 2 == 0;
    let vertical_edge = width / 2;
    let horizontal_edge = height / 2;
    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;
    if vertical_divider_inclusive {
        if horizontal_divider_inclusive {
            // nothing left out
            todo!();
        } else {
            // y has an excluded middle line, but not x
            todo!();
        }
    } else {
        if horizontal_divider_inclusive {
            // x has an excluded middle line, but not y
            todo!();
        } else {
            // both x and y have an excluded middle line
            // count q1
            for y in 0..horizontal_edge {
                for x in 0..vertical_edge {
                    match robot_map.get(&Coord { x, y }) {
                        Some(val) => {
                            q1_count += val;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            // count q2
            for y in 0..horizontal_edge {
                for x in (vertical_edge + 1)..width {
                    match robot_map.get(&Coord { x, y }) {
                        Some(val) => {
                            q2_count += val;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            // count q3
            for y in (horizontal_edge + 1)..height {
                for x in 0..vertical_edge {
                    match robot_map.get(&Coord { x, y }) {
                        Some(val) => {
                            q3_count += val;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            // count q4
            for y in (horizontal_edge + 1)..height {
                for x in (vertical_edge + 1)..width {
                    match robot_map.get(&Coord { x, y }) {
                        Some(val) => {
                            q4_count += val;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    //print_map(robot_map, width, height);
    //println!("q1: {q1_count}, q2: {q2_count}, q3: {q3_count}, q4: {q4_count}");
    (q1_count * q2_count * q3_count * q4_count).to_string()
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    //println!("{data:?}");
    process_map(101, 103, data, 100)
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
        let data = parse(file);
        assert_eq!(process_map(11, 7, data, 100), "12");
    }
}
