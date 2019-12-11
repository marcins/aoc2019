// day 3
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Cmd {
    dir: char,
    steps: i32,
}

fn trace_wire(wire: &Vec<Cmd>) -> HashMap<Point, i32> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    let mut points = HashMap::new();

    for cmd in wire {
        for _ in 0..cmd.steps {
            match cmd.dir {
                'U' => y -= 1,
                'L' => x -= 1,
                'D' => y += 1,
                'R' => x += 1,
                _ => panic!("Unexpected direction {}", cmd.dir),
            };
            steps += 1;
            let point = Point { x: x, y: y };
            points.insert(point, steps);
        }
    }
    return points;
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<Cmd>> {
    return input
        .split('\n')
        .map(|line| {
            line.split(",")
                .map(|cmd| cmd.split_at(1))
                .map(|(dir, count)| Cmd {
                    dir: dir.chars().next().unwrap(),
                    steps: count.parse::<i32>().unwrap(),
                })
                .collect()
        })
        .collect();
}

#[aoc(day3, part1)]
pub fn solve_part1(wires: &Vec<Vec<Cmd>>) -> i32 {
    println!("Tracing wire 1...");
    let points1 = trace_wire(&wires[0]);
    println!("Tracing wire 2...");
    let points2 = trace_wire(&wires[1]);
    println!("Finding overlaps...");
    let mut overlaps = points1.keys().fold(Vec::new(), |mut acc, point1| {
        if !points2.contains_key(point1) {
            return acc;
        }
        acc.push((
            point1.x.abs() + point1.y.abs(),
            points1.get(point1).unwrap() + points2.get(point1).unwrap(),
        ));
        return acc;
    });
    println!("Sorting...");
    overlaps.sort_by(|a, b| a.0.cmp(&b.0));
    let (distance, _) = overlaps[0];
    return distance;
}

#[aoc(day3, part2)]
pub fn solve_part2(wires: &Vec<Vec<Cmd>>) -> i32 {
    println!("Tracing wire 1...");
    let points1 = trace_wire(&wires[0]);
    println!("Tracing wire 2...");
    let points2 = trace_wire(&wires[1]);
    println!("Finding overlaps...");
    let mut overlaps = points1.keys().fold(Vec::new(), |mut acc, point1| {
        if !points2.contains_key(point1) {
            return acc;
        }
        acc.push((
            point1.x.abs() + point1.y.abs(),
            points1.get(point1).unwrap() + points2.get(point1).unwrap(),
        ));
        return acc;
    });
    println!("Sorting...");
    overlaps.sort_by(|a, b| a.1.cmp(&b.1));
    let (_, steps) = overlaps[0];
    return steps;
}
