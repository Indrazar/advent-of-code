use std::net::Incoming;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    multi::{many1, many_till},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

fn parse_button_a(input: &str) -> IResult<&str, Coord> {
    terminated(
        preceded(
            tag("Button A: X+"),
            separated_pair(complete::i64, tag(", Y+"), complete::i64).map(|(x, y)| Coord { x, y }),
        ),
        line_ending,
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, Coord> {
    terminated(
        preceded(
            tag("Button B: X+"),
            separated_pair(complete::i64, tag(", Y+"), complete::i64).map(|(x, y)| Coord { x, y }),
        ),
        line_ending,
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, Coord> {
    terminated(
        preceded(
            tag("Prize: X="),
            separated_pair(complete::i64, tag(", Y="), complete::i64).map(|(x, y)| Coord { x, y }),
        ),
        alt((line_ending, tag(""))),
    )(input)
}

fn parse_distant_prize(input: &str) -> IResult<&str, Coord> {
    terminated(
        preceded(
            tag("Prize: X="),
            separated_pair(complete::i64, tag(", Y="), complete::i64).map(|(x, y)| Coord {
                x: x + 10_000_000_000_000,
                y: y + 10_000_000_000_000,
            }),
        ),
        alt((line_ending, tag(""))),
    )(input)
}

fn parse_p1_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = parse_button_a(input)?;
    let (input, button_b) = parse_button_b(input)?;
    let (input, prize) = parse_prize(input)?;
    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_p2_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = parse_button_a(input)?;
    let (input, button_b) = parse_button_b(input)?;
    let (input, prize) = parse_distant_prize(input)?;
    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_p1(input: &str) -> Vec<Machine> {
    let (_, machines) =
        many1(many_till(anychar, parse_p1_machine).map(|(_discard, val)| val))(input)
            .expect("parse should succeed");
    machines
}

fn parse_p2(input: &str) -> Vec<Machine> {
    let (_, machines) =
        many1(many_till(anychar, parse_p2_machine).map(|(_discard, val)| val))(input)
            .expect("parse should succeed");
    machines
}

fn get_button_counts(machine: Machine) -> Option<(i64, i64)> {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    //     A   B
    // X: [94, 22  [A] = [8400]
    // Y:  34, 67] [B] = [5400]
    // *Button*X=*Result*
    // *Button*^-1*Result* = X
    // det([[a, b],[c, d]]) = 1/(ad-bc)
    // inverse([[a, b], [c,d]]) = 1/(ad-bc) [[d, -b], [-c, a]]
    let a = machine.button_a.x;
    let b = machine.button_b.x;
    let c = machine.button_a.y;
    let d = machine.button_b.y;
    if (a * d - b * c) == 0 {
        //println!("machine: {machine:?} has no inverse");
        return None;
    }
    let x = machine.prize.x;
    let y = machine.prize.y;
    // A = (dx-by)/(ad-bc)
    // B = -(cx-ay)/(ad-bc)
    // for bc < ad:
    // if by > dx then no positive integer solution
    // if ay < cx then no positive integer solution
    if b * c < a * d {
        if b * y > d * x {
            //println!("machine: {machine:?} has a negative solution");
            return None;
        }
        if a * y < c * x {
            //println!("machine: {machine:?} has a negative solution");
            return None;
        }
    }
    // for bc > ad:
    // if by < dx then no positive integer solution
    // if ay > cx then no positive integer solution
    else {
        if b * y < d * x {
            //println!("machine has a negative solution");
            return None;
        }
        if a * y > c * x {
            //println!("machine has a negative solution");
            return None;
        }
    }
    let det = a * d - b * c;
    let a_presses_numerator = (d * x) - (b * y);
    let a_presses = if a_presses_numerator % det != 0 {
        //println!("machine has a non-integer solution for A presses");
        return None;
    } else {
        a_presses_numerator / det
    };
    let b_presses_numerator = -((c * x) - (a * y));
    let b_presses = if b_presses_numerator % det != 0 {
        //println!("machine has a non-integer solution for B presses");
        return None;
    } else {
        b_presses_numerator / det
    };
    //println!("machine requires: A:{a_presses}, B:{b_presses}");
    return Some((a_presses, b_presses));
}

pub fn process_part1(input: &str) -> String {
    let a_cost = 3;
    let b_cost = 1;
    let machines = parse_p1(input);
    //println!("machines: {}\n{machines:?}", machines.len());
    let mut cost = 0;
    for machine in machines {
        match get_button_counts(machine) {
            Some((a, b)) => {
                cost += (a * a_cost) + (b * b_cost);
            }
            None => continue,
        }
    }
    cost.to_string()
}

pub fn process_part2(input: &str) -> String {
    let a_cost = 3;
    let b_cost = 1;
    let machines = parse_p2(input);
    //println!("machines: {}\n{machines:?}", machines.len());
    let mut cost = 0;
    for machine in machines {
        match get_button_counts(machine) {
            Some((a, b)) => {
                cost += (a * a_cost) + (b * b_cost);
            }
            None => continue,
        }
    }
    cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "480");
    }
}
