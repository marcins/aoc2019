// day 3
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    steps: i32,
}

fn trace_wire(wire: &Vec<&str>) -> HashSet<Point> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    let mut points = HashSet::new();

    for cmd in wire {
        let parts = cmd.split_at(1);
        let dir = parts.0;
        let count = parts.1.parse::<i32>().unwrap();
        for _ in 0..count {
            match dir {
                "U" => y -= 1,
                "L" => x -= 1,
                "D" => y += 1,
                "R" => x += 1,
                _ => panic!("Unexpected direction {}", dir),
            };
            steps += 1;
            points.insert(Point {
                x: x,
                y: y,
                steps: steps,
            });
        }
    }
    return points;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).unwrap();

    let wires: Vec<Vec<&str>> = input
        .split('\n')
        .map(|line| line.split(",").collect())
        .collect();

    println!("Tracing wire 1...");
    let points1 = trace_wire(&wires[0]);
    println!("Tracing wire 2...");
    let points2 = trace_wire(&wires[1]);
    println!("Finding overlaps...");
    let mut c = 0;
    let mut overlaps = points1.iter().fold(Vec::new(), |mut acc, point1| {
        c += 1;
        println!("{} {}", c, points1.len());
        let overlap = points2
            .iter()
            .find(|point2| point1.x == point2.x && point1.y == point2.y);
        if let Some(point2) = overlap {
            acc.push((point1.x.abs() + point1.y.abs(), point1.steps + point2.steps))
        }
        return acc;
    });
    println!("{:?}", overlaps);
    // let distances: Vec<(Point, i32)> = overlaps
    //     .map(|point| (point.x.abs() + point.y.abs()))
    //     .collect();

    overlaps.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", overlaps[0]);
}
