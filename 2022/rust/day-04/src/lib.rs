use nom::{bytes::complete::tag, character::complete, *};

struct LineData {
    left_start: u32,
    left_end: u32,
    right_start: u32,
    right_end: u32,
}

fn parse_line(input: &str) -> IResult<&str, LineData> {
    let (input, left_start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, left_end) = complete::u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, right_start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, right_end) = complete::u32(input)?;
    Ok((
        input,
        LineData {
            left_start,
            left_end,
            right_start,
            right_end,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            if let Ok((_, d)) = parse_line(line) {
                let left_size = d.left_end - d.left_start;
                let right_size = d.right_end - d.right_start;
                if left_size < right_size {
                    if d.left_start >= d.right_start && d.left_end <= d.right_end {
                        1
                    } else {
                        0
                    }
                } else {
                    // right is bigger
                    if d.right_start >= d.left_start && d.right_end <= d.left_end {
                        1
                    } else {
                        0
                    }
                }
            } else {
                0
            }
        })
        .sum::<usize>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            if let Ok((_, d)) = parse_line(line) {
                if d.left_end < d.right_start || d.right_end < d.left_start {
                    0
                } else {
                    1
                }
            } else {
                0
            }
        })
        .sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "2");
        assert_eq!(process_part2(file.as_str()), "4");
    }
}
