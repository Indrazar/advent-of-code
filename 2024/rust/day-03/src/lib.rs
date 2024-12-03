use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(i32, i32),
    Disable,
    Enable,
}

//fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {}

fn parse_mult_pair(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, (x, y)) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(x, y)))
}

fn parse_conditional_mult(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Disable, tag("don\'t()")),
        value(Instruction::Enable, tag("do()")),
        parse_mult_pair,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_mult_pair).map(|(_discard, ins)| ins))(input)
}

fn conditional_parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_conditional_mult).map(|(_discard, ins)| ins))(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, pairs) = parse(input).expect("parse should succeed");
    pairs
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(x, y) => x * y,
            Instruction::Enable => panic!("shouldn't get this"),
            Instruction::Disable => panic!("shouldn't get this"),
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut enabled = true;
    let (_, pairs) = conditional_parse(input).expect("parse should succeed");
    pairs
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(x, y) => match enabled {
                true => x * y,
                false => 0,
            },
            Instruction::Enable => {
                enabled = true;
                0
            }
            Instruction::Disable => {
                enabled = false;
                0
            }
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "161");
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part2(file), "48");
    }
}
