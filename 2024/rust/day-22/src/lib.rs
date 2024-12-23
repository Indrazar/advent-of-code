use std::collections::HashMap;

fn mix(current: i64, input: i64) -> i64 {
    current ^ input
}

fn prune(current: i64) -> i64 {
    current % 16777216
}

fn get_next(input: i64, map: &mut HashMap<i64, i64>) -> i64 {
    if let Some(v) = map.get(&input) {
        *v
    } else {
        // mix: mult 64, then prune
        let step1 = prune(mix(input, input * 64));
        let step2 = prune(mix(step1, step1 / 32));
        let step3 = prune(mix(step2, step2 * 2048));
        map.insert(input, step3);
        step3
    }
}

pub fn process_part1(input: &str) -> String {
    let mut map = HashMap::new();
    let mut buyer_secrets: Vec<i64> = Vec::new();
    for line in input.lines() {
        buyer_secrets.push(line.parse().expect("there should be a number"));
    }
    let mut final_values: Vec<i64> = Vec::new();
    for buyer in buyer_secrets {
        let mut current_secret = buyer;
        for _ in 0..2000 {
            current_secret = get_next(current_secret, &mut map);
        }
        final_values.push(current_secret);
    }
    final_values.iter().sum::<i64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut map = HashMap::new();
    let mut buyer_secrets: Vec<i64> = Vec::new();
    for line in input.lines() {
        buyer_secrets.push(line.parse().expect("there should be a number"));
    }
    //let mut all_diff_histories: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut all_best_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for buyer in buyer_secrets {
        let mut buyer_best_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        let mut current_secret = buyer;
        //let mut diff_history: Vec<(i64, i64)> = Vec::new();
        let (mut one, mut two, mut three, mut four): (
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
        ) = (None, None, None, None);
        let mut prev_price = current_secret % 10;
        for _ in 0..2000 {
            current_secret = get_next(current_secret, &mut map);
            let current_price = current_secret % 10;
            let diff = current_price - prev_price;
            //diff_history.push((diff, current_price));
            one = two;
            two = three;
            three = four;
            four = Some(diff);
            if let (Some(d1), Some(d2), Some(d3), Some(d4)) = (one, two, three, four) {
                buyer_best_map
                    .entry((d1, d2, d3, d4))
                    .or_insert(current_price);
            }
            prev_price = current_price;
        }
        //all_diff_histories.push(diff_history);
        for (key, value) in buyer_best_map {
            all_best_map
                .entry(key)
                .and_modify(|v| *v += value)
                .or_insert(value);
        }
    }
    all_best_map
        .values()
        .max()
        .expect("there should be a number")
        .to_string()
}

pub fn process_part2_slow(input: &str) -> String {
    let mut map = HashMap::new();
    let mut buyer_secrets: Vec<i64> = Vec::new();
    for line in input.lines() {
        buyer_secrets.push(line.parse().expect("there should be a number"));
    }
    let mut all_diff_histories: Vec<Vec<(i64, i64)>> = Vec::new();
    for buyer in buyer_secrets {
        let mut current_secret = buyer;
        let mut diff_history: Vec<(i64, i64)> = Vec::new();
        let mut current_price = current_secret % 10;
        for _ in 0..2000 {
            current_secret = get_next(current_secret, &mut map);
            diff_history.push((current_secret % 10 - current_price, current_secret % 10));
            current_price = current_secret % 10;
        }
        all_diff_histories.push(diff_history);
    }
    let mut bought_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for one in -9i64..9i64 {
        for two in -9i64..9i64 {
            for three in -9i64..9i64 {
                for four in -9i64..9i64 {
                    'next_buyer: for (buyer_number, buyer) in all_diff_histories.iter().enumerate()
                    {
                        for i in 3..buyer.len() {
                            if one == buyer[i - 3].0
                                && two == buyer[i - 2].0
                                && three == buyer[i - 1].0
                                && four == buyer[i].0
                            {
                                println!(
                                    "[{one}, {two}, {three}, {four}]: on iteration {i} bought {} from {}",
                                    buyer[i].1, buyer_number
                                );
                                bought_map
                                    .entry((one, two, three, four))
                                    .and_modify(|e| *e += buyer[i].1)
                                    .or_insert(buyer[i].1);
                                continue 'next_buyer;
                            }
                        }
                    }
                }
            }
        }
    }
    let max = bought_map.values().max().expect("there should be a max");
    for (k, v) in bought_map.iter() {
        if v == max {
            println!("{k:?}");
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut map = HashMap::new();
        let start = 123;
        let mut current = get_next(start, &mut map);
        let check = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for val in check {
            assert_eq!(current, val);
            current = get_next(current, &mut map);
        }
    }

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "37327623");
        let file = include_str!("../test-input-2.txt");
        //assert_eq!(process_part2_slow(file), "23");
        assert_eq!(process_part2(file), "23");
    }
}
