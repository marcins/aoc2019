use super::intcode;
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    return input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &Vec<i32>) -> i32 {
    let mut max_thrust = 0;

    for phase_a in 0..5 {
        for phase_b in 0..5 {
            for phase_c in 0..5 {
                for phase_d in 0..5 {
                    for phase_e in 0..5 {
                        // hack
                        let mut s = HashSet::new();
                        s.insert(phase_a);
                        s.insert(phase_b);
                        s.insert(phase_c);
                        s.insert(phase_d);
                        s.insert(phase_e);
                        // println!("{:?} {:?}", s, s.len());
                        if s.len() != 5 {
                            continue;
                        }

                        let a = intcode::execute(&input, &vec![phase_a, 0]).unwrap();
                        let b = intcode::execute(&input, &vec![phase_b, a]).unwrap();
                        let c = intcode::execute(&input, &vec![phase_c, b]).unwrap();
                        let d = intcode::execute(&input, &vec![phase_d, c]).unwrap();
                        let e = intcode::execute(&input, &vec![phase_e, d]).unwrap();
                        if e > max_thrust {
                            println!(
                                "{:?}{:?}{:?}{:?}{:?}",
                                phase_a, phase_b, phase_c, phase_d, phase_e,
                            );
                            max_thrust = e;
                        }
                    }
                }
            }
        }
    }
    max_thrust
}

#[aoc(day7, part2)]
pub fn day7_part2(input: &Vec<i32>) -> i32 {
    // Phase setting: 10342
    0
}
