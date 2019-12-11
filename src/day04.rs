fn is_candidate(pwd: String, pair_length_enforced: bool) -> bool {
    let mut is_increasing = true;
    let mut last_val: i32 = -1;
    let mut pair_length = 1;
    let mut pair_lengths: Vec<i32> = Vec::new();
    for c in pwd.chars() {
        let i = c.to_digit(10).unwrap() as i32;
        if last_val == i {
            pair_length += 1;
        } else {
            if pair_length > 1 {
                pair_lengths.push(pair_length);
            }
            pair_length = 1;
        }
        if i < last_val {
            is_increasing = false;
            break;
        }
        last_val = i;
    }

    if pair_length > 1 {
        pair_lengths.push(pair_length);
    }

    let pair_rule_passed = if pair_length_enforced {
        pair_lengths.iter().any(|&l| l == 2)
    } else {
        pair_lengths.len() > 0
    };

    return pair_rule_passed && is_increasing;
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<i32> {
    return input
        .split('-')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
}

#[aoc(day4, part1)]
pub fn day4_part1(range: &Vec<i32>) -> usize {
    println!("{:?}", range);
    let candidates: Vec<i32> = (range[0]..range[1])
        .filter(|pwd| is_candidate(pwd.to_string(), false))
        .collect();
    return candidates.len();
}

#[aoc(day4, part2)]
pub fn day4_part2(range: &Vec<i32>) -> usize {
    println!("{:?}", range);
    let candidates: Vec<i32> = (range[0]..range[1])
        .filter(|pwd| is_candidate(pwd.to_string(), true))
        .collect();
    return candidates.len();
}
