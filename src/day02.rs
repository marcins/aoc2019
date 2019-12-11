#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    return input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
}

#[aoc(day2, part1)]
pub fn solve_part1(program: &Vec<i32>) -> i32 {
    let mut part1_program = program.clone();
    part1_program[1] = 12;
    part1_program[2] = 2;

    let result = execute(part1_program).expect("Failed to execute");
    return result;
}

#[aoc(day2, part2)]
pub fn solve_part2(program: &Vec<i32>) -> i32 {
    let (noun, verb) = find_noun_verb(program, 19690720).expect("Oops, couldn't find a noun/verb!");
    return 100 * noun + verb;
}

fn execute(input_program: Vec<i32>) -> Result<i32, String> {
    let mut ptr = 0;
    let mut program = input_program.clone();

    loop {
        let opcode = program[ptr];

        let input1 = program[program[ptr + 1] as usize];
        let input2 = program[program[ptr + 2] as usize];
        let output = program[ptr + 3] as usize;

        let value;
        match opcode {
            1 => value = input1 + input2,
            2 => value = input1 * input2,
            99 => return Ok(program[0]),
            _ => return Err(format!("Invalid opcode: {}", opcode)),
        }
        program[output] = value;
        ptr += 4;
    }
}

fn find_noun_verb(program: &Vec<i32>, expected: i32) -> Result<(i32, i32), &'static str> {
    for noun in 1..99 {
        for verb in 1..99 {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            let result = execute(p).expect("Failed to execute");
            if result == expected {
                return Ok((noun, verb));
            }
        }
    }
    return Err("Unable to find a matching noun and verb!");
}
