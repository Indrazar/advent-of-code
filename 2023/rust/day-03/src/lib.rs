#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone, Copy)]
struct Label {
    value: u32,
    pos: Position,
    length: usize,
    is_part_number: IsPartNumber,
    part_location: PartData,
}

#[derive(Debug, Clone, Copy)]
struct GearData {
    label1: u32,
    label2: u32,
    pos: Position,
}

#[derive(Copy, Clone, Debug)]
enum PartData {
    Unchecked,
    None,
    Present(Symbol),
}

#[derive(Copy, Clone, Debug)]
enum IsPartNumber {
    Unchecked,
    True,
    False,
}
#[derive(Debug, Clone, Copy)]
struct Symbol {
    symbol: char,
    pos: Position,
}

fn check_if_label_is_part_number(
    label: &mut Label,
    schematic: &Vec<Vec<Symbol>>,
    lines: usize,
    cols: usize,
) -> bool {
    let y_start = label.pos.y;
    let x_start = label.pos.x;
    let label_width = label.length;
    // now we walk around each item in the label_list and look for clues I mean parts
    // start at left of first letter
    if x_start != 0 && x_start - 1 < cols && y_start < lines {
        if !(schematic[y_start][x_start - 1].symbol.is_alphanumeric())
            && (schematic[y_start][x_start - 1].symbol != '.')
        {
            label.part_location = PartData::Present(schematic[y_start][x_start - 1].clone());
            return true;
        }
    }
    // go up one
    if x_start != 0 && x_start - 1 <= cols && y_start != 0 && y_start - 1 < lines {
        if !(schematic[y_start - 1][x_start - 1].symbol.is_alphanumeric())
            && (schematic[y_start - 1][x_start - 1].symbol != '.')
        {
            label.part_location = PartData::Present(schematic[y_start - 1][x_start - 1].clone());
            return true;
        }
    }
    // go down one
    if x_start != 0 && x_start - 1 <= cols && y_start + 1 < lines {
        if !(schematic[y_start + 1][x_start - 1].symbol.is_alphanumeric())
            && (schematic[y_start + 1][x_start - 1].symbol != '.')
        {
            label.part_location = PartData::Present(schematic[y_start + 1][x_start - 1].clone());
            return true;
        }
    }
    let mut walk = 0;
    while walk <= label_width {
        // we need to go all the way past to the diagnal above and to the right so <= not <
        if x_start + walk < cols && y_start != 0 && y_start - 1 < lines {
            if !(schematic[y_start - 1][x_start + walk]
                .symbol
                .is_alphanumeric())
                && (schematic[y_start - 1][x_start + walk].symbol != '.')
            {
                label.part_location =
                    PartData::Present(schematic[y_start - 1][x_start + walk].clone());
                return true;
            }
        }
        walk += 1;
    }
    // now we do square on the right
    if x_start + label_width < cols && y_start < lines {
        if !(schematic[y_start][x_start + label_width]
            .symbol
            .is_alphanumeric())
            && (schematic[y_start][x_start + label_width].symbol != '.')
        {
            label.part_location =
                PartData::Present(schematic[y_start][x_start + label_width].clone());
            return true;
        }
    }
    // now do the bottom
    walk = 0;
    while walk <= label_width {
        if x_start + walk < cols && y_start + 1 < lines {
            if !(schematic[y_start + 1][x_start + walk]
                .symbol
                .is_alphanumeric())
                && (schematic[y_start + 1][x_start + walk].symbol != '.')
            {
                label.part_location = PartData::Present(schematic[y_start + 1][x_start + walk]);
                return true;
            }
        }
        walk += 1;
    }
    label.part_location = PartData::None;
    false
}

fn get_line_width(input: &str) -> usize {
    input
        .lines()
        .next()
        .expect("at least one line of input")
        .len()
}

fn build_schematic(input: &str) -> Vec<Vec<Symbol>> {
    let line_width = get_line_width(input);
    // input sanity checks, not needed since they pass
    //for line in input.lines() {
    //    if line.len() > line_width {
    //        panic!("input is malformed, line too wide");
    //    }
    //}
    let mut schematic: Vec<Vec<Symbol>> = Vec::default();
    // schematic[y][x] or schematic[line][col]
    let mut line_num: usize = 0;
    for line in input.lines() {
        schematic.push(Vec::default());
        for (col, character) in line.chars().enumerate() {
            schematic[line_num].push(Symbol {
                symbol: character,
                pos: Position {
                    y: line_num,
                    x: col,
                },
            });
        }
        line_num += 1;
    }
    let total_lines = line_num;
    // schematic built, now parse for all the labels
    let mut label_list: Vec<Label> = Vec::default();
    let mut line_iter = schematic.iter();
    while let Some(line) = line_iter.next() {
        let mut symbol_iter = line.iter().peekable();
        while let Some(symbol) = symbol_iter.next() {
            if symbol.symbol.is_numeric() {
                let y_pos = symbol.pos.y;
                let x_pos = symbol.pos.x;
                let mut new_label_chars: Vec<char> = Vec::default();
                new_label_chars.push(symbol.symbol);
                while symbol_iter
                    .peek()
                    .is_some_and(|v| (**v).symbol.is_numeric())
                {
                    new_label_chars.push(
                        symbol_iter
                            .next()
                            .expect("peek failure? something should be here")
                            .symbol,
                    );
                }
                let label: String = new_label_chars.iter().collect();
                label_list.push(Label {
                    value: label.parse().expect("this should be a number"),
                    pos: Position { y: y_pos, x: x_pos },
                    length: label.len(),
                    is_part_number: IsPartNumber::Unchecked,
                    part_location: PartData::Unchecked,
                })
            }
        }
    }
    schematic
}

fn build_label_list(schematic: &Vec<Vec<Symbol>>) -> Vec<Label> {
    let mut label_list: Vec<Label> = Vec::default();
    let mut line_iter = schematic.iter();
    while let Some(line) = line_iter.next() {
        let mut symbol_iter = line.iter().peekable();
        while let Some(symbol) = symbol_iter.next() {
            if symbol.symbol.is_numeric() {
                let y_pos = symbol.pos.y;
                let x_pos = symbol.pos.x;
                let mut new_label_chars: Vec<char> = Vec::default();
                new_label_chars.push(symbol.symbol);
                while symbol_iter
                    .peek()
                    .is_some_and(|v| (**v).symbol.is_numeric())
                {
                    new_label_chars.push(
                        symbol_iter
                            .next()
                            .expect("peek failure? something should be here")
                            .symbol,
                    );
                }
                let label: String = new_label_chars.iter().collect();
                label_list.push(Label {
                    value: label.parse().expect("this should be a number"),
                    pos: Position { y: y_pos, x: x_pos },
                    length: label.len(),
                    is_part_number: IsPartNumber::Unchecked,
                    part_location: PartData::Unchecked,
                })
            }
        }
    }
    label_list
}

pub fn process_part1(input: &str) -> String {
    let line_width = get_line_width(input);
    let schematic: Vec<Vec<Symbol>> = build_schematic(input);
    // schematic built, now parse for all the labels
    let mut label_list: Vec<Label> = build_label_list(&schematic);
    // now we walk around each item in the label_list and look for clues I mean parts
    for label in label_list.iter_mut() {
        match check_if_label_is_part_number(label, &schematic, schematic.len(), line_width) {
            true => label.is_part_number = IsPartNumber::True,
            false => label.is_part_number = IsPartNumber::False,
        }
    }
    label_list
        .iter()
        .map(|label| match label.is_part_number {
            IsPartNumber::Unchecked => panic!("should have been checked"),
            IsPartNumber::True => label.value,
            IsPartNumber::False => 0,
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let line_width = get_line_width(input);
    let schematic: Vec<Vec<Symbol>> = build_schematic(input);
    // schematic built, now parse for all the labels
    let mut label_list: Vec<Label> = build_label_list(&schematic);
    // now we walk around each item in the label_list and look for clues I mean parts
    for label in label_list.iter_mut() {
        match check_if_label_is_part_number(label, &schematic, schematic.len(), line_width) {
            true => label.is_part_number = IsPartNumber::True,
            false => label.is_part_number = IsPartNumber::False,
        }
    }
    let mut possible_gears: Vec<GearData> = Vec::default();
    //determine all possible gears
    for label in label_list {
        match label.part_location {
            PartData::Unchecked => panic!("all parts should have been checked"),
            PartData::None => continue,
            PartData::Present(data) => {
                if data.symbol == '*' {
                    let mut gear_found = false;
                    for gear in possible_gears.iter_mut() {
                        if gear.pos == data.pos {
                            gear.label2 = label.value;
                            gear_found = true;
                            break;
                        } else {
                            continue;
                        }
                    }
                    if gear_found {
                        continue;
                    } else {
                        possible_gears.push(GearData {
                            label1: label.value,
                            label2: 0,
                            pos: data.pos,
                        })
                    }
                }
            }
        }
    }
    possible_gears
        .iter()
        .map(|gear| {
            if gear.label2 == 0 {
                0
            } else {
                gear.label1 * gear.label2
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "4361");
        assert_eq!(process_part2(file.as_str()), "467835")
    }
}
