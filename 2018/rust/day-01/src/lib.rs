use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    multi::{many1, many_till},
    sequence::terminated,
    IResult, Parser,
};

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (input, sign) = alt((tag("-"), tag("+")))(input)?;
    let (input, num) = complete::i64(input)?;
    let num = match sign {
        "-" => -num,
        "+" => num,
        _ => panic!("invalid sign"),
    };
    Ok((input, num))
}

fn parse(input: &str) -> Vec<i64> {
    let (_, res) = many1(terminated(parse_number, alt((line_ending, tag("")))))(input)
        .expect("parse should succeed");
    res
}

pub fn process_part1(input: &str) -> String {
    let changes = parse(input);
    let mut sum = 0;
    for change in changes {
        sum += change;
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let changes = parse(input);
    let mut sum = 0;
    let mut history = vec![sum];
    let mut changes_iter = changes.iter();
    loop {
        sum += match changes_iter.next() {
            Some(val) => val,
            None => {
                changes_iter = changes.iter();
                changes_iter.next().unwrap()
            }
        };
        if history.contains(&sum) {
            return sum.to_string();
        } else {
            history.push(sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "-3");
    }
}
