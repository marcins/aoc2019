fn required_fuel(mass: i64) -> i64 {
    (((mass as f32) / 3.0).floor() as i64) - 2
}

fn fuel_mass(mass: i64) -> i64 {
    let mut fuel_mass = 0;
    let mut fuel = required_fuel(mass);
    while fuel > 0 {
        fuel_mass += fuel;
        fuel = required_fuel(fuel);
    }
    fuel_mass
}

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(modules: &Vec<i64>) -> i64 {
    modules
        .iter()
        .fold(0, |total_fuel, module| total_fuel + required_fuel(*module))
}

#[aoc(day1, part2)]
pub fn solve_part2(modules: &Vec<i64>) -> i64 {
    modules
        .iter()
        .fold(0, |total_fuel, module| total_fuel + fuel_mass(*module))
}
