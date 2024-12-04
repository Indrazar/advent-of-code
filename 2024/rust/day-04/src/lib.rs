fn build_search(input: &str) -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = Vec::default();
    for line in input.lines() {
        let mut search_line: Vec<char> = Vec::default();
        for ch in line.chars() {
            search_line.push(ch);
        }
        word_search.push(search_line);
    }
    word_search
}

fn build_search_and_blank_heatmap(input: &str) -> (Vec<Vec<char>>, Vec<Vec<i32>>) {
    let mut word_search: Vec<Vec<char>> = Vec::default();
    let mut heat_map: Vec<Vec<i32>> = Vec::default();
    for line in input.lines() {
        let mut search_line: Vec<char> = Vec::default();
        let mut heat_map_line: Vec<i32> = Vec::default();
        for ch in line.chars() {
            search_line.push(ch);
            heat_map_line.push(0);
        }
        word_search.push(search_line);
        heat_map.push(heat_map_line);
    }
    (word_search, heat_map)
}

pub fn process_part1(input: &str) -> String {
    let mut count = 0;
    #[cfg(debug_assertions)]
    let (word_search, mut heat_map) = build_search_and_blank_heatmap(input);
    #[cfg(not(debug_assertions))]
    let word_search = build_search(input);
    let mut char1: char = '.';
    let mut char2: char = '.';
    let mut char3: char = '.';
    //row
    for y in 0..word_search.len() {
        for x in 3..word_search[0].len() {
            if x == 3 {
                char1 = word_search[y][x - 3];
                char2 = word_search[y][x - 2];
                char3 = word_search[y][x - 1];
            }
            let word = [char1, char2, char3, word_search[y][x]]
                .iter()
                .collect::<String>();
            if word.as_str() == "XMAS" || word.as_str() == "SAMX" {
                #[cfg(debug_assertions)]
                {
                    println!("found -- at {x},{y}");
                    heat_map[y][x - 3] += 1;
                    heat_map[y][x - 2] += 1;
                    heat_map[y][x - 1] += 1;
                    heat_map[y][x] += 1;
                }
                count += 1;
            }
            char1 = char2;
            char2 = char3;
            char3 = word_search[y][x];
        }
    }
    //column
    for x in 0..word_search[0].len() {
        for y in 3..word_search.len() {
            if y == 3 {
                char1 = word_search[y - 3][x];
                char2 = word_search[y - 2][x];
                char3 = word_search[y - 1][x];
            }

            let word = [char1, char2, char3, word_search[y][x]]
                .iter()
                .collect::<String>();
            if word.as_str() == "XMAS" || word.as_str() == "SAMX" {
                #[cfg(debug_assertions)]
                {
                    println!("found | at {x},{y}");
                    heat_map[y - 3][x] += 1;
                    heat_map[y - 2][x] += 1;
                    heat_map[y - 1][x] += 1;
                    heat_map[y][x] += 1;
                }
                count += 1;
            }
            char1 = char2;
            char2 = char3;
            char3 = word_search[y][x];
        }
    }
    //diag '\'
    for y in 3..word_search.len() {
        for x in 3..word_search[0].len() {
            char1 = word_search[y - 3][x - 3];
            char2 = word_search[y - 2][x - 2];
            char3 = word_search[y - 1][x - 1];

            let word = [char1, char2, char3, word_search[y][x]]
                .iter()
                .collect::<String>();
            if word.as_str() == "XMAS" || word.as_str() == "SAMX" {
                #[cfg(debug_assertions)]
                {
                    println!("found diag \\ at {x},{y}");
                    heat_map[y - 3][x - 3] += 1;
                    heat_map[y - 2][x - 2] += 1;
                    heat_map[y - 1][x - 1] += 1;
                    heat_map[y][x] += 1;
                }
                count += 1;
            }
        }
    }
    //diag '/'
    for y in 3..word_search.len() {
        for x in 3..word_search[0].len() {
            let char1 = word_search[y - 3][x];
            let char2 = word_search[y - 2][x - 1];
            let char3 = word_search[y - 1][x - 2];
            let char4 = word_search[y][x - 3];

            let word = [char1, char2, char3, char4].iter().collect::<String>();
            if word.as_str() == "XMAS" || word.as_str() == "SAMX" {
                #[cfg(debug_assertions)]
                {
                    println!("found / at {x},{y}");
                    heat_map[y - 3][x] += 1;
                    heat_map[y - 2][x - 1] += 1;
                    heat_map[y - 1][x - 2] += 1;
                    heat_map[y][x - 3] += 1;
                }
                count += 1;
            }
        }
    }
    #[cfg(debug_assertions)]
    dbg!(count);
    #[cfg(debug_assertions)]
    {
        for row in heat_map {
            for val in row {
                print!("{val}");
            }
            print!("\n");
        }
    }
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut count: i32 = 0;
    let word_search = build_search(input);
    for y in 2..word_search.len() {
        for x in 2..word_search[0].len() {
            let val_ul_corner = word_search[y - 2][x - 2];
            let val_ur_corner = word_search[y - 2][x];
            let middle = word_search[y - 1][x - 1];
            let val_ll_corner = word_search[y][x - 2];
            let val_lr_corner = word_search[y][x];
            if middle == 'A'
                && ((val_ul_corner == 'M' && val_lr_corner == 'S')
                    || (val_ul_corner == 'S' && val_lr_corner == 'M'))
                && ((val_ur_corner == 'M' && val_ll_corner == 'S')
                    || (val_ur_corner == 'S' && val_ll_corner == 'M'))
            {
                count += 1;
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_negative6() {
        let file = include_str!("../test-input--6.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_negative5() {
        let file = include_str!("../test-input--5.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_negative4() {
        let file = include_str!("../test-input--4.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_negative3() {
        let file = include_str!("../test-input--3.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_negative2() {
        let file = include_str!("../test-input--2.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_negative1() {
        let file = include_str!("../test-input--1.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test1_0() {
        let file = include_str!("../test-input-0.txt");
        assert_eq!(process_part1(file), "4");
    }
    #[test]
    fn test1_1() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "18");
    }
    #[test]
    fn test1_2() {
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part1(file), "18");
    }
    #[test]
    fn test2_1() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part2(file), "9");
    }
    #[test]
    fn test2_3() {
        let file = include_str!("../test-input-3.txt");
        assert_eq!(process_part2(file), "1");
    }
    #[test]
    fn test2_4() {
        let file = include_str!("../test-input-4.txt");
        assert_eq!(process_part2(file), "2");
    }
    #[test]
    fn test2_5() {
        let file = include_str!("../test-input-5.txt");
        assert_eq!(process_part2(file), "1");
    }
    #[test]
    fn test2_6() {
        let file = include_str!("../test-input-6.txt");
        assert_eq!(process_part2(file), "1");
    }
    #[test]
    fn test2_7() {
        let file = include_str!("../test-input-7.txt");
        assert_eq!(process_part2(file), "4");
    }
    #[test]
    fn test2_8() {
        let file = include_str!("../test-input-8.txt");
        assert_eq!(process_part2(file), "9")
    }
}
