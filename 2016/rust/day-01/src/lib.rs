use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char},
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    L(i64),
    R(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse_instr(input: &str) -> IResult<&str, Instruction> {
    let (input, dir) = alt((char('L'), char('R')))(input)?;
    let (input, val) = complete::i64(input)?;
    match dir {
        'L' => Ok((input, Instruction::L(val))),
        'R' => Ok((input, Instruction::R(val))),
        unknown => panic!("incorrect instruction type found: {unknown}"),
    }

    //.map(|s: (&str, i64)| -> Instruction { match s.0 {} })
}

fn parse(input: &str) -> Vec<Instruction> {
    let (_, instr) = many1(terminated(parse_instr, alt((tag(", "), tag("")))))(input)
        .expect("parse should succeed");
    instr
}

fn turn_left(input: Direction) -> Direction {
    match input {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn turn_right(input: Direction) -> Direction {
    match input {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn advance(position: &Coord, direction: &Direction, count: i64) -> Coord {
    match direction {
        Direction::North => Coord {
            x: position.x,
            y: position.y - count,
        },
        Direction::East => Coord {
            x: position.x + count,
            y: position.y,
        },
        Direction::South => Coord {
            x: position.x,
            y: position.y + count,
        },
        Direction::West => Coord {
            x: position.x - count,
            y: position.y,
        },
    }
}

pub fn process_part1(input: &str) -> String {
    let steps = parse(input);
    let mut direction = Direction::North;
    let mut position = Coord { x: 0, y: 0 };
    for step in steps {
        match step {
            Instruction::L(count) => {
                direction = turn_left(direction);
                position = advance(&position, &direction, count);
            }
            Instruction::R(count) => {
                direction = turn_right(direction);
                position = advance(&position, &direction, count)
            }
        }
    }
    (position.x.abs() + position.y.abs()).to_string()
}

fn advance_with_history(
    mut position: Coord,
    direction: &Direction,
    count: i64,
    history: &mut Vec<Coord>,
) -> (Coord, bool) {
    for _ in 0..count {
        position = match direction {
            Direction::North => Coord {
                x: position.x,
                y: position.y - 1,
            },
            Direction::East => Coord {
                x: position.x + 1,
                y: position.y,
            },
            Direction::South => Coord {
                x: position.x,
                y: position.y + 1,
            },
            Direction::West => Coord {
                x: position.x - 1,
                y: position.y,
            },
        };
        if !history.contains(&position) {
            history.push(position);
        } else {
            return (position, true);
        }
    }
    (position, false)
}

pub fn process_part2(input: &str) -> String {
    let steps = parse(input);
    let mut direction = Direction::North;
    let mut position = Coord { x: 0, y: 0 };
    let mut history: Vec<Coord> = vec![position];
    let mut leave: bool;
    for step in steps {
        match step {
            Instruction::L(count) => {
                direction = turn_left(direction);
                (position, leave) = advance_with_history(position, &direction, count, &mut history);
                if leave {
                    break;
                }
            }
            Instruction::R(count) => {
                direction = turn_right(direction);
                (position, leave) = advance_with_history(position, &direction, count, &mut history);
                if leave {
                    break;
                }
            }
        }
    }
    (position.x.abs() + position.y.abs()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(process_part1("R2, L3"), "5");
        assert_eq!(process_part1("R2, R2, R2"), "2");
        assert_eq!(process_part1("R5, L5, R5, R3"), "12");
        assert_eq!(process_part2("R8, R4, R4, R8"), "4")
    }
}
