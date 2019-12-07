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

fn find_noun_verb(program: Vec<i32>, expected: i32) -> Result<(i32, i32), &'static str> {
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

fn main() {
    let program = vec![
        1, 0, 0, 3, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 9, 19, 23, 1, 13, 23, 27, 1,
        5, 27, 31, 2, 31, 6, 35, 1, 35, 5, 39, 1, 9, 39, 43, 1, 43, 5, 47, 1, 47, 5, 51, 2, 10, 51,
        55, 1, 5, 55, 59, 1, 59, 5, 63, 2, 63, 9, 67, 1, 67, 5, 71, 2, 9, 71, 75, 1, 75, 5, 79, 1,
        10, 79, 83, 1, 83, 10, 87, 1, 10, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99, 2, 99, 9, 103, 1,
        103, 6, 107, 1, 13, 107, 111, 1, 13, 111, 115, 2, 115, 9, 119, 1, 119, 6, 123, 2, 9, 123,
        127, 1, 127, 5, 131, 1, 131, 5, 135, 1, 135, 5, 139, 2, 10, 139, 143, 2, 143, 10, 147, 1,
        147, 5, 151, 1, 151, 2, 155, 1, 155, 13, 0, 99, 2, 14, 0, 0,
    ];

    let mut part1_program = program.clone();
    part1_program[1] = 12;
    part1_program[2] = 2;

    let result = execute(part1_program).expect("Failed to execute");
    println!("Part 1: {:?}", result);

    let (noun, verb) = find_noun_verb(program, 19690720).expect("Oops, couldn't find a noun/verb!");
    println!("Part 2: {} {} {}", noun, verb, 100 * noun + verb)
}
