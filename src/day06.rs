use std::collections::{HashMap, HashSet};
use std::usize;

#[aoc_generator(day6)]
pub fn generator(inp: &str) -> Vec<Vec<String>> {
    inp.split('\n')
        .map(|line| line.split(')').map(|s| String::from(s.trim())).collect())
        .collect()
}

fn build_paths<'a>(
    curr_path: &Vec<&'a String>,
    paths: &mut HashSet<Vec<&'a String>>,
    orbits: &HashMap<&'a String, &'a String>,
) {
    let curr = curr_path.last().unwrap();

    match orbits.get(curr) {
        Some(orbiting) => {
            let mut next_path = curr_path.clone();
            next_path.push(orbiting);
            paths.insert(next_path.clone());
            build_paths(&next_path, paths, orbits);
        }
        None => {
            return;
        }
    }
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &Vec<Vec<String>>) -> usize {
    let mut orbits = HashMap::new();
    let mut objects = HashSet::new();

    for orbit in input {
        let orbiting = &orbit[0];
        let orbiter = &orbit[1];

        orbits.insert(orbiter, orbiting);
        objects.insert(orbiter);
        objects.insert(orbiting);
    }

    let mut paths: HashSet<Vec<&String>> = HashSet::new();
    for leaf in objects {
        let curr_path = vec![leaf];
        build_paths(&curr_path, &mut paths, &orbits);
    }

    paths.len()
}

#[aoc(day6, part2)]
pub fn day6_part2(input: &Vec<Vec<String>>) -> usize {
    let mut orbits = HashMap::new();
    let mut objects = HashSet::new();

    for orbit in input {
        let orbiting = &orbit[0];
        let orbiter = &orbit[1];

        orbits.insert(orbiter, orbiting);
        objects.insert(orbiter);
        objects.insert(orbiting);
    }

    let mut paths: HashSet<Vec<&String>> = HashSet::new();
    for leaf in objects {
        let curr_path = vec![leaf];
        build_paths(&curr_path, &mut paths, &orbits);
    }

    let src = orbits.get(&String::from("YOU")).unwrap();
    let tgt = orbits.get(&String::from("SAN")).unwrap();

    let paths_from_src: Vec<&Vec<&String>> = paths
        .iter()
        .filter(|path| path.first().unwrap() == src)
        .collect();

    let paths_from_tgt: Vec<&Vec<&String>> = paths
        .iter()
        .filter(|path| path.first().unwrap() == tgt)
        .collect();

    let mut min = usize::MAX;
    for path in paths_from_src.iter() {
        for node in path.iter() {
            let hops_from_src = match path.iter().position(|t| t == node) {
                Some(hops) => {
                    if hops > min {
                        continue;
                    }
                    hops
                }
                None => continue,
            };
            let candidates: Vec<_> = paths_from_tgt
                .iter()
                .filter(|tgt_path| tgt_path.contains(node))
                .collect();
            if candidates.len() > 0 {
                // common point is node -> how many hops from src to node + tgt to node
                let hops = candidates
                    .iter()
                    .map(|c| c.iter().position(|t| t == node))
                    .filter(|hops| hops.is_some())
                    .map(|hops| hops.unwrap() + hops_from_src)
                    .min()
                    .unwrap();
                if hops < min {
                    min = hops;
                }
            }
        }
    }

    min
}
